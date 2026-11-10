//! # `laus-core` — Local-First Offline State Engine (`~ L@u’s Skill Labs`)
//!
//! Pure Rust domain and synchronization engine designed for both `wasm32-unknown-unknown`
//! and native desktop targets with zero UI or I/O coupling.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

/// Canonical project identifier preserved across serialized snapshots.
pub const PROJECT_IDENTIFIER: &str = "~ L@u’s Skill Labs (~l@u / ~l4u)";

/// Errors produced by the Local-First state engine.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("document key must not be empty")]
    EmptyDocumentKey,
    #[error("outbox entry {0} not found")]
    OutboxEntryNotFound(u64),
    #[error("snapshot serialization failed: {0}")]
    SerializationError(String),
}

/// Monotonic Lamport logical clock for causal ordering across offline nodes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Default)]
pub struct LamportClock {
    pub tick: u64,
}

impl LamportClock {
    /// Creates a new Lamport clock starting at tick `0`.
    pub const fn new() -> Self {
        Self { tick: 0 }
    }

    /// Advances the clock for a local mutation event.
    pub fn increment(&mut self) -> u64 {
        self.tick = self.tick.saturating_add(1);
        self.tick
    }

    /// Merges an incoming remote Lamport timestamp and advances by 1.
    pub fn witness(&mut self, remote_tick: u64) -> u64 {
        self.tick = self.tick.max(remote_tick).saturating_add(1);
        self.tick
    }
}

/// Lifecycle state of a transactional Outbox mutation entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutboxStatus {
    /// Queued locally, awaiting network connectivity.
    Pending,
    /// Currently being transmitted to remote peer.
    InFlight { attempts: u32 },
    /// Confirmed persisted by remote peer at the given revision.
    Synced { remote_revision: u64 },
    /// Failed with a transient network error; scheduled for backoff retry.
    FailedRetryable {
        attempts: u32,
        backoff_ms: u64,
        last_error: String,
    },
}

/// An immutable mutation record persisted alongside local state changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutboxEntry {
    pub entry_id: u64,
    pub lamport_tick: u64,
    pub node_id: String,
    pub document_key: String,
    pub payload_json: String,
    pub status: OutboxStatus,
}

impl OutboxEntry {
    /// Computes deterministic exponential backoff in milliseconds (`base_ms * 2^attempts`, capped).
    pub fn compute_backoff_ms(attempts: u32, base_ms: u64, max_ms: u64) -> u64 {
        let shift = attempts.min(16);
        let multiplier = 1_u64.checked_shl(shift).unwrap_or(u64::MAX);
        base_ms.saturating_mul(multiplier).min(max_ms)
    }
}

/// Domain entity representing a Local-First document or skill artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LabDocument {
    pub key: String,
    pub title: String,
    pub body_markdown: String,
    pub lamport_tick: u64,
    pub author_node: String,
}

impl LabDocument {
    /// Resolves concurrent offline edits deterministically using Last-Write-Wins (LWW)
    /// ordered by `(lamport_tick, author_node)`.
    pub fn resolve_lww<'a>(&'a self, incoming: &'a Self) -> &'a Self {
        match incoming.lamport_tick.cmp(&self.lamport_tick) {
            std::cmp::Ordering::Greater => incoming,
            std::cmp::Ordering::Less => self,
            std::cmp::Ordering::Equal => {
                if incoming.author_node >= self.author_node {
                    incoming
                } else {
                    self
                }
            }
        }
    }
}

/// Complete serializable snapshot of the offline state engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EngineSnapshot {
    pub project_id: String,
    pub node_id: String,
    pub clock: LamportClock,
    pub documents: BTreeMap<String, LabDocument>,
    pub outbox: Vec<OutboxEntry>,
}

/// Deterministic Local-First state engine managing documents and the transactional Outbox.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OfflineStateEngine {
    node_id: String,
    clock: LamportClock,
    next_outbox_id: u64,
    documents: BTreeMap<String, LabDocument>,
    outbox: Vec<OutboxEntry>,
}

impl OfflineStateEngine {
    /// Initializes a new offline engine for the specified logical node identifier.
    pub fn new(node_id: impl Into<String>) -> Self {
        Self {
            node_id: node_id.into(),
            clock: LamportClock::new(),
            next_outbox_id: 1,
            documents: BTreeMap::new(),
            outbox: Vec::new(),
        }
    }

    /// Returns the logical node identifier.
    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    /// Returns the current Lamport clock value.
    pub fn current_tick(&self) -> u64 {
        self.clock.tick
    }

    /// Returns an immutable view of all stored documents.
    pub fn documents(&self) -> &BTreeMap<String, LabDocument> {
        &self.documents
    }

    /// Returns all entries in the transactional Outbox.
    pub fn outbox(&self) -> &[OutboxEntry] {
        &self.outbox
    }

    /// Counts how many Outbox entries require synchronization (`Pending` or `FailedRetryable`).
    pub fn pending_sync_count(&self) -> usize {
        self.outbox
            .iter()
            .filter(|e| {
                matches!(
                    e.status,
                    OutboxStatus::Pending | OutboxStatus::FailedRetryable { .. }
                )
            })
            .count()
    }

    /// Applies an optimistic local document mutation and atomically appends a `Pending` Outbox entry.
    pub fn upsert_document(
        &mut self,
        key: impl Into<String>,
        title: impl Into<String>,
        body_markdown: impl Into<String>,
    ) -> Result<u64, CoreError> {
        let key = key.into();
        if key.trim().is_empty() {
            return Err(CoreError::EmptyDocumentKey);
        }

        let tick = self.clock.increment();
        let doc = LabDocument {
            key: key.clone(),
            title: title.into(),
            body_markdown: body_markdown.into(),
            lamport_tick: tick,
            author_node: self.node_id.clone(),
        };

        let payload_json = serde_json::to_string(&doc)
            .map_err(|e| CoreError::SerializationError(e.to_string()))?;

        self.documents.insert(key.clone(), doc);

        let entry_id = self.next_outbox_id;
        self.next_outbox_id = self.next_outbox_id.saturating_add(1);

        self.outbox.push(OutboxEntry {
            entry_id,
            lamport_tick: tick,
            node_id: self.node_id.clone(),
            document_key: key,
            payload_json,
            status: OutboxStatus::Pending,
        });

        Ok(entry_id)
    }

    /// Merges a remote document using deterministic LWW conflict resolution and witnesses its clock.
    pub fn merge_remote_document(&mut self, remote_doc: LabDocument) {
        self.clock.witness(remote_doc.lamport_tick);
        match self.documents.get(&remote_doc.key) {
            Some(existing) => {
                let winner = existing.resolve_lww(&remote_doc).clone();
                self.documents.insert(remote_doc.key, winner);
            }
            None => {
                self.documents.insert(remote_doc.key.clone(), remote_doc);
            }
        }
    }

    /// Marks an Outbox entry as `Synced` once acknowledged by a remote peer.
    pub fn acknowledge_synced(
        &mut self,
        entry_id: u64,
        remote_revision: u64,
    ) -> Result<(), CoreError> {
        let entry = self
            .outbox
            .iter_mut()
            .find(|e| e.entry_id == entry_id)
            .ok_or(CoreError::OutboxEntryNotFound(entry_id))?;
        entry.status = OutboxStatus::Synced { remote_revision };
        Ok(())
    }

    /// Records a transient network failure on an Outbox entry and computes its retry backoff.
    pub fn record_sync_failure(
        &mut self,
        entry_id: u64,
        error_message: impl Into<String>,
    ) -> Result<u64, CoreError> {
        let entry = self
            .outbox
            .iter_mut()
            .find(|e| e.entry_id == entry_id)
            .ok_or(CoreError::OutboxEntryNotFound(entry_id))?;

        let prev_attempts = match entry.status {
            OutboxStatus::InFlight { attempts }
            | OutboxStatus::FailedRetryable { attempts, .. } => attempts,
            _ => 0,
        };
        let attempts = prev_attempts.saturating_add(1);
        let backoff_ms = OutboxEntry::compute_backoff_ms(attempts, 500, 60_000);

        entry.status = OutboxStatus::FailedRetryable {
            attempts,
            backoff_ms,
            last_error: error_message.into(),
        };
        Ok(backoff_ms)
    }

    /// Exports the entire engine state and Outbox log as a deterministic JSON snapshot.
    pub fn export_snapshot_json(&self) -> Result<String, CoreError> {
        let snapshot = EngineSnapshot {
            project_id: PROJECT_IDENTIFIER.to_string(),
            node_id: self.node_id.clone(),
            clock: self.clock,
            documents: self.documents.clone(),
            outbox: self.outbox.clone(),
        };
        serde_json::to_string_pretty(&snapshot)
            .map_err(|e| CoreError::SerializationError(e.to_string()))
    }

    /// Restores an `OfflineStateEngine` from a serialized JSON snapshot.
    pub fn import_snapshot_json(json: &str) -> Result<Self, CoreError> {
        let snapshot: EngineSnapshot =
            serde_json::from_str(json).map_err(|e| CoreError::SerializationError(e.to_string()))?;
        let next_outbox_id = snapshot
            .outbox
            .iter()
            .map(|e| e.entry_id)
            .max()
            .unwrap_or(0)
            .saturating_add(1);

        Ok(Self {
            node_id: snapshot.node_id,
            clock: snapshot.clock,
            next_outbox_id,
            documents: snapshot.documents,
            outbox: snapshot.outbox,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimistic_upsert_increments_clock_and_enqueues_outbox() {
        let mut engine = OfflineStateEngine::new("node-lau-01");
        let entry_id = engine
            .upsert_document("skill/local-first", "Local-First Core", "# ~ L@u Notes")
            .expect("upsert should succeed");

        assert_eq!(entry_id, 1);
        assert_eq!(engine.current_tick(), 1);
        assert_eq!(engine.pending_sync_count(), 1);
        assert_eq!(engine.documents().len(), 1);
    }

    #[test]
    fn outbox_backoff_and_sync_lifecycle() {
        let mut engine = OfflineStateEngine::new("node-lau-01");
        let id = engine
            .upsert_document("doc-1", "Offline Spec", "Content")
            .unwrap();

        let backoff = engine
            .record_sync_failure(id, "network unreachable")
            .unwrap();
        assert_eq!(backoff, 1000);
        assert_eq!(engine.pending_sync_count(), 1);

        engine.acknowledge_synced(id, 42).unwrap();
        assert_eq!(engine.pending_sync_count(), 0);
    }

    #[test]
    fn lww_conflict_resolution_converges_deterministically() {
        let mut node_a = OfflineStateEngine::new("node-a");
        node_a
            .upsert_document("shared-key", "Version A", "Edited on A")
            .unwrap();

        let remote_higher = LabDocument {
            key: "shared-key".to_string(),
            title: "Version B".to_string(),
            body_markdown: "Edited on B at higher tick".to_string(),
            lamport_tick: 10,
            author_node: "node-b".to_string(),
        };

        node_a.merge_remote_document(remote_higher);
        let resolved = node_a.documents().get("shared-key").unwrap();
        assert_eq!(resolved.title, "Version B");
        assert_eq!(node_a.current_tick(), 11);
    }

    #[test]
    fn snapshot_round_trip_preserves_state() {
        let mut engine = OfflineStateEngine::new("node-lau-snapshot");
        engine
            .upsert_document("k1", "Title 1", "Body 1")
            .expect("upsert ok");

        let json = engine.export_snapshot_json().expect("export ok");
        let restored = OfflineStateEngine::import_snapshot_json(&json).expect("import ok");

        assert_eq!(restored.node_id(), "node-lau-snapshot");
        assert_eq!(restored.current_tick(), 1);
        assert_eq!(restored.documents().len(), 1);
        assert_eq!(restored.outbox().len(), 1);
    }
}

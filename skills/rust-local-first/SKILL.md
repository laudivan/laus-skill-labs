---
name: rust-local-first
description: Architect and implement Local-First Rust domain engines (`crates/core`) with deterministic state machines, Lamport clocks, transactional Outbox queues, and zero-I/O platform independence for native and `wasm32-unknown-unknown` targets.
---

# Skill: Rust Local-First State Engine (`rust-local-first`)

Use this skill when designing or modifying core domain logic in **~ L@u’s Skill Labs** (`~l@u` / `~l4u`) or any Local-First Rust application that must operate seamlessly offline across WebAssembly and native targets.

---

## 1. Core Architectural Rules

1. **Strict Platform Agnosticism**:
   - The core crate (`crates/core`) must never depend on DOM APIs (`web-sys`), UI frameworks (`egui`), or OS windowing (`wry`/`tao`).
   - All timestamps or clocks must be injected or represented as monotonic logical counters (`LamportClock` / `u64` revision) so state transitions are 100% deterministic in unit tests.
2. **Optimistic Local Mutation + Transactional Outbox**:
   - Every state change is applied immediately to the local state store and simultaneously appended as an `OutboxEntry` in the same in-memory transaction.
   - Never block a UI command on a network Future.
3. **Deterministic Conflict Resolution**:
   - Use Last-Write-Wins (LWW) registers ordered by `(lamport_tick, node_id)` so concurrent offline edits on two devices converge to the exact same state regardless of message arrival order.

---

## 2. Reference Pattern: State Engine + Outbox

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OutboxStatus {
    Pending,
    InFlight { attempts: u32 },
    Synced { remote_rev: u64 },
    FailedRetryable { attempts: u32, last_error: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutboxEntry {
    pub entry_id: u64,
    pub lamport_tick: u64,
    pub entity_key: String,
    pub payload_json: String,
    pub status: OutboxStatus,
}
```

---

## 3. Verification Checklist

Before completing any task involving `rust-local-first`:
- [ ] Run `cargo test -p laus-core` and verify offline mutations enqueue Outbox items deterministically.
- [ ] Run `cargo check -p laus-core --target wasm32-unknown-unknown` to confirm zero platform-specific leakage.
- [ ] Verify outbox retry and conflict tie-breaking tests cover concurrent Lamport timestamps.

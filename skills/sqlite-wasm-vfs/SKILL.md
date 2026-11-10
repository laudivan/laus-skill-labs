---
name: sqlite-wasm-vfs
description: Configure and operate SQLite in `wasm32-unknown-unknown` over the Origin Private File System (OPFS) synchronous Virtual File System (VFS) with IndexedDB block-level fallback and WAL-mode local durability.
---

# Skill: SQLite WebAssembly VFS over OPFS & IndexedDB (`sqlite-wasm-vfs`)

Use this skill when implementing persistent relational storage for Local-First Rust/Wasm applications in **~ L@u’s Skill Labs** (`~l@u` / `~l4u`).

---

## 1. Architecture of SQLite in `wasm32-unknown-unknown`

Standard `wasm32-unknown-unknown` lacks POSIX `open`/`read`/`write`/`fsync` syscalls. To run SQLite at near-native speed inside a browser or `wry` WebView without server round-trips, register a custom **SQLite VFS** backed by:

1. **Primary Tier — OPFS (`FileSystemSyncAccessHandle`)**:
   - Runs inside a dedicated Web Worker where `createSyncAccessHandle()` exposes synchronous byte-range reads (`read(buffer, { at })`) and writes (`write(buffer, { at })`, `flush()`).
   - Required HTTP/custom-protocol headers for SharedArrayBuffer & OPFS coordination:
     - `Cross-Origin-Opener-Policy: same-origin`
     - `Cross-Origin-Embedder-Policy: require-corp`
2. **Fallback Tier — IndexedDB Page Store (`idb-vfs`)**:
   - Used when running on main thread or restricted environments without COOP/COEP headers.
   - Stores fixed-size (`4096`-byte) SQLite database pages as IndexedDB records keyed by `(db_name, page_number)` with an in-memory write-through page cache.

---

## 2. Recommended SQLite Pragmas for Wasm VFS

Configure every local SQLite connection with deterministic, low-overhead pragmas optimized for single-user Local-First storage:

```sql
PRAGMA page_size = 4096;
PRAGMA journal_mode = WAL;
PRAGMA synchronous = NORMAL;
PRAGMA temp_store = MEMORY;
PRAGMA foreign_keys = ON;
```

---

## 3. Schema Pattern for Outbox + Domain State

```sql
CREATE TABLE IF NOT EXISTS domain_entities (
    entity_key TEXT PRIMARY KEY NOT NULL,
    lamport_tick INTEGER NOT NULL,
    node_id TEXT NOT NULL,
    payload_json TEXT NOT NULL,
    updated_at_ms INTEGER NOT NULL
) STRICT;

CREATE TABLE IF NOT EXISTS outbox_log (
    entry_id INTEGER PRIMARY KEY AUTOINCREMENT,
    lamport_tick INTEGER NOT NULL,
    entity_key TEXT NOT NULL,
    operation TEXT NOT NULL,
    payload_json TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    attempts INTEGER NOT NULL DEFAULT 0
) STRICT;
```

---

## 4. Verification Checklist

- [ ] Ensure `laus-launcher` and dev servers inject `Cross-Origin-Opener-Policy: same-origin` and `Cross-Origin-Embedder-Policy: require-corp`.
- [ ] Request persistent storage quota via `navigator.storage.persist()` on PWA initialization to prevent browser storage eviction.
- [ ] Verify graceful fallback to IndexedDB page blocks when `FileSystemSyncAccessHandle` is unavailable.

---
title: "Strategic Goals & Vision - ~ L@u’s Skill Labs"
author: "Laudivan Freire de Almeida (~ L@u / Uncle Lau)"
status: "Active"
---

# Strategic Goals — ~ L@u’s Skill Labs (`~l@u` / `~l4u`)

This document outlines the short-, medium-, and long-term architectural and engineering goals of **~ L@u’s Skill Labs**, led by **Laudivan Freire de Almeida (~ L@u)**.

---

## 1. Short-Term Goals (Foundation & Deterministic Offline Core)

- **Strict Crate Separation**: Establish a clean Cargo workspace (`crates/core`, `crates/ui-desktop`, `crates/ui-mobile`, `crates/launcher`) where business logic in `crates/core` has zero dependency on DOM, WebView, or OS windowing APIs.
- **Transactional Outbox Engine**: Implement an in-memory and serializable Local-First state engine supporting optimistic mutations, monotonic Lamport/vector timestamps, and an idempotent Outbox queue.
- **Dual-Surface `egui` Ergonomics**:
  - Deliver a high-density desktop layout (`crates/ui-desktop`) optimized for mouse precision and keyboard shortcuts.
  - Deliver a touch-first mobile layout (`crates/ui-mobile`) enforcing a strict minimum interactive target size of `>= 48.0px` (`48x48` logical pixels).
- **Standalone PWA & Cache-First Service Worker**: Provide a zero-network-latency startup shell (`web-shell/`) with hardware form-factor detection, a Cache-First Service Worker (`sw.js`), and `Trunk.toml` automation.
- **Agentic Build & Audit Server**: Ship `mcp-servers/rust-pwa-builder` exposing Model Context Protocol (MCP) tools to build Wasm targets and audit PWA offline readiness automatically.

---

## 2. Medium-Term Goals (Persistence VFS & Native Packaging)

- **SQLite Wasm VFS Integration**: Wire `crates/core` persistence adapters to an Origin Private File System (OPFS) synchronous VFS on modern browsers with automatic fallback to IndexedDB block storage.
- **Zero-Runtime Desktop Distribution**: Compile the `web-shell/dist` output directly into `crates/launcher` via `include_dir!`, producing single-file native executables (`< 10 MB`) for macOS, Linux, and FreeBSD using `wry` and `tao`.
- **Background Sync & Exponential Backoff**: Implement automatic Outbox flushing via Web Background Sync APIs and native network-reachability monitors with jittered exponential backoff and dead-letter inspection.
- **Automated Performance & Accessibility Telemetry**: Extend `rust-pwa-builder` MCP tools to verify Wasm binary size budgets (`wasm-opt`) and validate `>= 48px` touch target compliance statically and at runtime.

---

## 3. Long-Term Goals (Distributed Local-First Ecosystem)

- **CRDT & Peer-to-Peer Sync**: Introduce state-based and operation-based CRDT merge strategies over WebRTC and local LAN mDNS discovery for serverless multi-device synchronization between desktop (`launcher`) and mobile PWAs.
- **Encrypted Local Storage at Rest**: Add client-side authenticated encryption (XChaCha20-Poly1305 / Argon2id key derivation) so synced outbox logs and OPFS SQLite pages remain zero-knowledge on untrusted relay servers.
- **Composable Agentic Skill Library**: Expand `skills/` into a comprehensive reference suite for autonomous coding agents building high-reliability Rust + Wasm + Native hybrid applications under the **~ L@u** standard.

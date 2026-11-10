---
title: "Engineering Roadmap & Tasklist - ~ L@u’s Skill Labs"
author: "Laudivan Freire de Almeida (~ L@u / Uncle Lau)"
---

# Engineering Roadmap (`TODO.md`) — ~ L@u’s Skill Labs

Structured roadmap tracking foundational milestones and upcoming engineering tasks for **~ L@u’s Skill Labs** (`~l@u` / `~l4u`).

---

## Phase 1: Governance, Architecture & Skill Specifications
- [x] Initialize repository structure and feature-branch Git governance workflow
- [x] Author root documentation (`README.md`, `GOALS.md`, `TODO.md`, `CONTRIBUTING.md`, `CHANGELOG.md`, `LICENSE`) with transparent human + AI attribution
- [x] Define strict crate boundary and target rules in `guidelines/architecture.md`
- [x] Specify IndexedDB/OPFS storage matrix and Outbox pattern in `guidelines/offline-cache-policy.md`
- [x] Document ultralight `wry`/`tao` embedded desktop wrapper in `guidelines/launcher-spec.md`
- [x] Publish reusable AI agent skills (`rust-local-first`, `egui-wasm-pwa`, `sqlite-wasm-vfs`)

## Phase 2: Cargo Workspace & Dual-Surface `egui` Scaffold
- [x] Configure root Cargo workspace (`crates/core`, `crates/ui-desktop`, `crates/ui-mobile`, `crates/launcher`)
- [x] Implement Local-First state engine, Outbox queue, and sync status models in `crates/core`
- [x] Implement dense multi-panel desktop state view in `crates/ui-desktop`
- [x] Implement touch-ergonomic mobile view (`>= 48px` targets) in `crates/ui-mobile`
- [x] Implement `crates/launcher` native binary with embedded `web-shell` asset resolver (`include_dir!`)

## Phase 3: Web PWA Shell & MCP Builder Server
- [x] Create responsive `web-shell/index.html` with viewport/touch detection and dynamic Wasm bundle loader
- [x] Configure standalone `web-shell/manifest.json` and `web-shell/Trunk.toml`
- [x] Implement Cache-First `web-shell/sw.js` Service Worker with offline asset pre-caching
- [x] Implement `mcp-servers/rust-pwa-builder` MCP server with `build_wasm_target` and `audit_offline_readiness` tools

## Phase 4: Next-Generation Persistence & Sync Enhancements
- [ ] Integrate OPFS-backed `sqlite-wasm-vfs` page store adapter into `crates/core`
- [ ] Add `wasm-bindgen` bridge connecting browser `online`/`offline` events to `crates/core` sync coordinator
- [ ] Add automated `wasm-opt -Oz` post-build hook in `Trunk.toml`
- [ ] Implement end-to-end headless browser offline simulation tests for `sw.js`
- [ ] Package signed native desktop bundles for macOS (`.app`), Linux (`AppImage`), and FreeBSD

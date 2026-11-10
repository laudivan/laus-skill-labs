# Changelog

All notable changes to **~ L@u’s Skill Labs** (`~l@u` / `~l4u`) will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-11-10

### Added
- **Governance & Project Documentation**: Initialized `README.md`, `GOALS.md`, `TODO.md`, `CONTRIBUTING.md`, `CHANGELOG.md`, and `LICENSE` with transparent co-authorship attribution for **Laudivan Freire de Almeida (~ L@u / Uncle Lau)**, **Gemini**, and **Antigravity**.
- **Architectural Guidelines (`guidelines/`)**:
  - `architecture.md`: Defined strict crate boundaries and dual-target (`wasm32-unknown-unknown` + native) compilation architecture.
  - `offline-cache-policy.md`: Defined the IndexedDB/OPFS persistence matrix, Outbox queue lifecycle, and online exception recovery policies.
  - `launcher-spec.md`: Specified the zero-external-runtime native desktop wrapper using `wry`, `tao`, and `include_dir!`.
- **Agentic Engineering Skills (`skills/`)**:
  - `skills/rust-local-first/SKILL.md`: Patterns for offline state machines, vector clocks, and transactional outbox queues in Rust.
  - `skills/egui-wasm-pwa/SKILL.md`: Patterns for dual-surface adaptive `egui` applications compiled to WebAssembly PWAs.
  - `skills/sqlite-wasm-vfs/SKILL.md`: Patterns for running SQLite over OPFS and IndexedDB VFS in `wasm32-unknown-unknown`.
- **Cargo Workspace (`crates/`)**:
  - `crates/core` (`laus-core`): Pure Rust offline state engine, Outbox mutation queue, deterministic state snapshots, and sync state machine.
  - `crates/ui-desktop` (`laus-ui-desktop`): High-density multi-panel `egui` desktop interface for large displays and mouse/keyboard navigation.
  - `crates/ui-mobile` (`laus-ui-mobile`): Responsive touch-first `egui` interface enforcing `>= 48px` minimum hit targets.
  - `crates/launcher` (`laus-launcher`): Ultralight native desktop binary embedding `web-shell/` assets via `include_dir!` with `wry`/`tao` integration.
- **Progressive Web App Shell (`web-shell/`)**:
  - `index.html`: Responsive bootstrapper with hardware/viewport detection switching between `ui-desktop` and `ui-mobile` Wasm bundles.
  - `manifest.json`: Standalone PWA configuration with offline-ready metadata.
  - `sw.js`: Cache-First Service Worker ensuring instant offline startup and deterministic asset caching.
  - `Trunk.toml`: Automated Wasm compilation and asset bundling configuration.
- **Model Context Protocol Server (`mcp-servers/rust-pwa-builder/`)**:
  - Functional Rust MCP server implementing JSON-RPC 2.0 over stdio and CLI diagnostics for building Wasm targets and auditing PWA offline readiness.

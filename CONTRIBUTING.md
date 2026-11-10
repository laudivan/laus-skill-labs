---
title: "Contributing Guide - ~ L@u’s Skill Labs"
author: "Laudivan Freire de Almeida (~ L@u / Uncle Lau)"
---

# Contributing to ~ L@u’s Skill Labs (`~l@u` / `~l4u`)

Thank you for contributing to **~ L@u’s Skill Labs**! Whether you are a human engineer or an autonomous AI coding agent, please follow these technical standards to keep the repository clean, deterministic, and verifiable.

---

## 1. Language & Naming Conventions

- **Primary Repository Language**: All technical documentation, code comments, docstrings, identifiers (variables, functions, structs, enums, traits, modules), commit messages, and file names **must be written in English**.
- **Brand Identifier Exception**: Always preserve the project author's canonical identifiers—`~ L@u`, `~l@u`, `~l4u`, `Uncle Lau`, and `Tio Lau`—exactly as written.
- **File Naming**:
  - Markdown & Documentation files: `kebab-case` (e.g., `architecture.md`, `offline-cache-policy.md`, `launcher-spec.md`) with the exception of standard root governance files (`README.md`, `GOALS.md`, `TODO.md`, `CONTRIBUTING.md`, `CHANGELOG.md`, `SKILL.md`).
  - Rust source files & modules: `snake_case` (e.g., `lib.rs`, `main.rs`, `state_engine.rs`).

---

## 2. Rust Code Standards & Quality Gates

All Rust code across the root workspace and `mcp-servers/` must pass the following checks before merging:

1. **Formatting (`rustfmt`)**:
   ```bash
   cargo fmt --all -- --check
   ```
2. **Static Analysis (`clippy`)**:
   ```bash
   cargo clippy --workspace --all-targets -- -D warnings
   ```
3. **Cross-Target Verification**:
   - `crates/core`, `crates/ui-desktop`, and `crates/ui-mobile` must compile cleanly for both the host native target and `wasm32-unknown-unknown`:
     ```bash
     cargo check --workspace
     cargo check -p laus-core -p laus-ui-desktop -p laus-ui-mobile --target wasm32-unknown-unknown
     ```
4. **Unit & Integration Tests**:
   ```bash
   cargo test --workspace
   ```

---

## 3. Architectural Boundaries

- **`crates/core` Purity**: Never import platform-specific windowing (`wry`, `tao`) or UI rendering (`egui`) crates inside `crates/core` unless gated behind explicit feature flags.
- **Mobile Touch Ergonomics**: Every interactive widget in `crates/ui-mobile` must enforce a minimum touch target dimension of **`48.0 x 48.0` logical pixels** (`MIN_TOUCH_TARGET_PX = 48.0`).
- **Offline-First Invariant**: State mutations must always succeed locally via the Outbox engine before any network synchronization is attempted.

---

## 4. Git Branch Governance & Semantic Commits

Do not commit feature work directly onto `main`. Always work on a dedicated feature branch and merge back into `main`.

### Branch Naming
- `feature/<module-or-capability>` — New architecture modules, crates, skills, or shells (e.g., `feature/cargo-workspace-scaffold`)
- `fix/<issue-summary>` — Bug fixes or offline recovery patches
- `docs/<topic>` — Documentation and guideline updates
- `chore/<task>` — Tooling, CI, or dependency maintenance

### Semantic Commit Messages (Conventional Commits)
Format all commit messages as `<type>(<optional-scope>): <imperative summary in English>`:
- `feat:` — A new feature, crate, skill, or capability
- `fix:` — A bug fix
- `docs:` — Documentation, guidelines, or skill specs only
- `refactor:` — Code restructuring without changing external behavior
- `test:` — Adding or updating unit/integration tests
- `chore:` — Build scripts, workspace configuration, or maintenance

---

## 5. Attribution Transparency

When contributing via or alongside AI agents, maintain full transparency in commit notes or documentation credits as established by **Laudivan Freire de Almeida (~ L@u / Uncle Lau)**.

---
name: egui-wasm-pwa
description: Build adaptive dual-surface `egui` user interfaces (`crates/ui-desktop` and `crates/ui-mobile`) compiled to WebAssembly PWAs with Trunk, enforcing high-density desktop workspaces and `>= 48px` mobile touch ergonomics.
---

# Skill: Adaptive Dual-Surface `egui` Wasm PWA (`egui-wasm-pwa`)

Use this skill when building or updating user interfaces in **~ L@u’s Skill Labs** (`~l@u` / `~l4u`) targeting both desktop mouse/keyboard environments and mobile touchscreens via `egui` and WebAssembly.

---

## 1. Dual-Surface Ergonomic Standards

### Desktop Surface (`crates/ui-desktop`)
- **Target Input**: Mouse cursor (`pointer: fine`) + hardware keyboard shortcuts.
- **Layout**: Multi-panel workspace (`SidePanel` inspector + `TopBottomPanel` telemetry bar + `CentralPanel` dense table/canvas).
- **Density**: Compact spacing (`item_spacing = vec2(8.0, 6.0)`), rich status indicators, and keyboard accelerators (`Cmd/Ctrl+K`, `Cmd/Ctrl+S`).

### Mobile Surface (`crates/ui-mobile`)
- **Target Input**: Coarse touch (`pointer: coarse`, `maxTouchPoints > 0`) and narrow viewports (`< 768px`).
- **Mandatory Touch Target Invariant**: Every interactive widget (`Button`, tab selector, sync trigger, list item action) **must enforce a minimum size of `48.0 x 48.0` logical pixels** (`egui::vec2(width.max(48.0), 48.0)`).
- **Layout**: Single-column vertical scroll flow with primary navigation anchored at the bottom thumb zone.

---

## 2. Applying `egui::Context` Ergonomics

```rust
use egui::{Context, Vec2};

pub const MIN_TOUCH_TARGET_PX: f32 = 48.0;

pub fn configure_mobile_style(ctx: &Context) {
    let mut style = (*ctx.style()).clone();
    style.spacing.interact_size = Vec2::new(MIN_TOUCH_TARGET_PX, MIN_TOUCH_TARGET_PX);
    style.spacing.item_spacing = Vec2::new(10.0, 12.0);
    style.spacing.button_padding = Vec2::new(16.0, 12.0);
    ctx.set_style(style);
}
```

---

## 3. PWA Integration & Device Switching (`web-shell/`)

- `web-shell/index.html` detects the client form factor before initializing WebAssembly:
  - Desktop browsers load the `laus-ui-desktop` Wasm bundle.
  - Touch/mobile devices load the `laus-ui-mobile` Wasm bundle.
- `web-shell/sw.js` caches both the shell HTML/manifest and compiled `.wasm`/`.js` artifacts using a **Cache-First** strategy so cold boots succeed offline.

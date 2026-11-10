//! # `laus-ui-mobile` — Touch-First Responsive `egui` Layer (`~ L@u’s Skill Labs`)
//!
//! Designed for mobile devices, tablets, and coarse-pointer PWAs. Enforces a strict
//! minimum interactive touch target of `>= 48.0px` (`48x48` logical pixels).

use egui::{Context, Vec2};
use laus_core::{OfflineStateEngine, PROJECT_IDENTIFIER};
use serde::{Deserialize, Serialize};

/// Mandatory minimum touch target size in logical pixels (WCAG / Material guidelines).
pub const MIN_TOUCH_TARGET_PX: f32 = 48.0;

/// Primary bottom-navigation views optimized for one-handed thumb reach.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MobileScreen {
    Cards,
    QuickCapture,
    SyncStatus,
}

/// Responsive touch-first application state wrapping [`OfflineStateEngine`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileTouchApp {
    engine: OfflineStateEngine,
    active_screen: MobileScreen,
    quick_title: String,
    quick_note: String,
}

impl MobileTouchApp {
    /// Initializes a new touch-first mobile surface backed by [`OfflineStateEngine`].
    pub fn new(engine: OfflineStateEngine) -> Self {
        Self {
            engine,
            active_screen: MobileScreen::Cards,
            quick_title: "Mobile Lab Note".to_string(),
            quick_note: "Captured offline on ~ L@u Mobile PWA".to_string(),
        }
    }

    /// Returns a reference to the underlying [`OfflineStateEngine`].
    pub fn engine(&self) -> &OfflineStateEngine {
        &self.engine
    }

    /// Configures `egui::Context` style to guarantee `>= 48.0px` touch target dimensions.
    pub fn configure_touch_style(ctx: &Context) {
        let mut style = (*ctx.style()).clone();
        style.spacing.interact_size = Vec2::new(MIN_TOUCH_TARGET_PX, MIN_TOUCH_TARGET_PX);
        style.spacing.item_spacing = Vec2::new(10.0, 12.0);
        style.spacing.button_padding = Vec2::new(16.0, 12.0);
        ctx.set_style(style);
    }

    /// Helper that constructs a touch-compliant button with at least `MIN_TOUCH_TARGET_PX` height/width.
    pub fn touch_button(
        label: impl Into<egui::WidgetText>,
        min_width: f32,
    ) -> egui::Button<'static> {
        egui::Button::new(label).min_size(Vec2::new(
            min_width.max(MIN_TOUCH_TARGET_PX),
            MIN_TOUCH_TARGET_PX,
        ))
    }

    /// Renders a single frame of the touch-first mobile interface.
    pub fn render(&mut self, ctx: &Context) {
        Self::configure_touch_style(ctx);

        egui::TopBottomPanel::top("laus_mobile_header").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.strong(PROJECT_IDENTIFIER);
                ui.label(format!("Pending: {}", self.engine.pending_sync_count()));
            });
        });

        egui::TopBottomPanel::bottom("laus_mobile_bottom_nav").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                if ui.add(Self::touch_button("Cards", 96.0)).clicked() {
                    self.active_screen = MobileScreen::Cards;
                }
                if ui.add(Self::touch_button("Capture", 96.0)).clicked() {
                    self.active_screen = MobileScreen::QuickCapture;
                }
                if ui.add(Self::touch_button("Outbox", 96.0)).clicked() {
                    self.active_screen = MobileScreen::SyncStatus;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match self.active_screen {
            MobileScreen::Cards => {
                ui.heading("Offline Lab Cards");
                for doc in self.engine.documents().values() {
                    ui.group(|ui| {
                        ui.strong(&doc.title);
                        ui.label(&doc.body_markdown);
                        ui.small(format!("Rev v{} ({})", doc.lamport_tick, doc.author_node));
                    });
                }
            }
            MobileScreen::QuickCapture => {
                ui.heading("Quick Offline Capture");
                ui.text_edit_singleline(&mut self.quick_title);
                ui.text_edit_multiline(&mut self.quick_note);
                if ui
                    .add(Self::touch_button("Save to Local Outbox", 200.0))
                    .clicked()
                {
                    let key = format!("mobile/note-{}", self.engine.current_tick() + 1);
                    let _ = self.engine.upsert_document(
                        key,
                        self.quick_title.clone(),
                        self.quick_note.clone(),
                    );
                    self.active_screen = MobileScreen::Cards;
                }
            }
            MobileScreen::SyncStatus => {
                ui.heading("Transactional Outbox");
                ui.label(format!(
                    "Node: {} | Clock: {}",
                    self.engine.node_id(),
                    self.engine.current_tick()
                ));
                for entry in self.engine.outbox() {
                    ui.label(format!(
                        "#{} {} -> {:?}",
                        entry.entry_id, entry.document_key, entry.status
                    ));
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mobile_touch_targets_enforce_minimum_48px_invariant() {
        const { assert!(MIN_TOUCH_TARGET_PX >= 48.0) };

        let engine = OfflineStateEngine::new("mobile-node-01");
        let mut app = MobileTouchApp::new(engine);
        let ctx = Context::default();

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            app.render(ctx);
        });

        let interact_size = ctx.style().spacing.interact_size;
        assert!(interact_size.x >= 48.0);
        assert!(interact_size.y >= 48.0);
    }
}

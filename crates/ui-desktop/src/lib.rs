//! # `laus-ui-desktop` — High-Density Desktop `egui` Layer (`~ L@u’s Skill Labs`)
//!
//! Optimized for large screens (`>= 1024px`), fine mouse pointers, multi-column split panels,
//! and keyboard-driven workflows.

use egui::{Context, Vec2};
use laus_core::{OfflineStateEngine, PROJECT_IDENTIFIER};
use serde::{Deserialize, Serialize};

/// Compact spacing constants tailored for high-density desktop monitors.
pub const DESKTOP_ITEM_SPACING_X: f32 = 8.0;
pub const DESKTOP_ITEM_SPACING_Y: f32 = 6.0;
pub const DESKTOP_ROW_HEIGHT_PX: f32 = 24.0;

/// Active inspector panel in the desktop workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DesktopInspectorTab {
    DocumentTree,
    OutboxQueue,
    SyncTelemetry,
}

/// High-density desktop application state wrapping [`OfflineStateEngine`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopWorkspaceApp {
    engine: OfflineStateEngine,
    active_tab: DesktopInspectorTab,
    selected_document_key: Option<String>,
    draft_key: String,
    draft_title: String,
    draft_body: String,
    show_outbox_drawer: bool,
}

impl DesktopWorkspaceApp {
    /// Creates a new desktop workspace backed by a fresh or restored [`OfflineStateEngine`].
    pub fn new(engine: OfflineStateEngine) -> Self {
        Self {
            engine,
            active_tab: DesktopInspectorTab::DocumentTree,
            selected_document_key: None,
            draft_key: "skill/new-artifact".to_string(),
            draft_title: "New Skill Artifact".to_string(),
            draft_body: "# ~ L@u Desktop Note\n\nLocal-first state ready.".to_string(),
            show_outbox_drawer: true,
        }
    }

    /// Returns a reference to the underlying [`OfflineStateEngine`].
    pub fn engine(&self) -> &OfflineStateEngine {
        &self.engine
    }

    /// Returns a mutable reference to the underlying [`OfflineStateEngine`].
    pub fn engine_mut(&mut self) -> &mut OfflineStateEngine {
        &mut self.engine
    }

    /// Applies dense desktop styling to the `egui::Context`.
    pub fn configure_desktop_style(ctx: &Context) {
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = Vec2::new(DESKTOP_ITEM_SPACING_X, DESKTOP_ITEM_SPACING_Y);
        style.spacing.interact_size = Vec2::new(40.0, DESKTOP_ROW_HEIGHT_PX);
        style.spacing.button_padding = Vec2::new(8.0, 4.0);
        ctx.set_style(style);
    }

    /// Renders a single frame of the multi-panel desktop workspace.
    pub fn render(&mut self, ctx: &Context) {
        Self::configure_desktop_style(ctx);

        egui::TopBottomPanel::top("laus_desktop_top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.strong(PROJECT_IDENTIFIER);
                ui.separator();
                ui.label(format!("Node: {}", self.engine.node_id()));
                ui.separator();
                ui.label(format!("Lamport Tick: {}", self.engine.current_tick()));
                ui.separator();
                ui.label(format!(
                    "Outbox Pending: {}",
                    self.engine.pending_sync_count()
                ));
            });
        });

        egui::SidePanel::left("laus_desktop_sidebar")
            .default_width(240.0)
            .show(ctx, |ui| {
                ui.heading("Explorer");
                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(
                            self.active_tab == DesktopInspectorTab::DocumentTree,
                            "Docs",
                        )
                        .clicked()
                    {
                        self.active_tab = DesktopInspectorTab::DocumentTree;
                    }
                    if ui
                        .selectable_label(
                            self.active_tab == DesktopInspectorTab::OutboxQueue,
                            "Outbox",
                        )
                        .clicked()
                    {
                        self.active_tab = DesktopInspectorTab::OutboxQueue;
                    }
                    if ui
                        .selectable_label(
                            self.active_tab == DesktopInspectorTab::SyncTelemetry,
                            "Telemetry",
                        )
                        .clicked()
                    {
                        self.active_tab = DesktopInspectorTab::SyncTelemetry;
                    }
                });
                ui.separator();

                match self.active_tab {
                    DesktopInspectorTab::DocumentTree => {
                        for (key, doc) in self.engine.documents() {
                            let is_selected = self.selected_document_key.as_deref() == Some(key);
                            if ui
                                .selectable_label(
                                    is_selected,
                                    format!("{} (v{})", doc.title, doc.lamport_tick),
                                )
                                .clicked()
                            {
                                self.selected_document_key = Some(key.clone());
                            }
                        }
                    }
                    DesktopInspectorTab::OutboxQueue => {
                        for entry in self.engine.outbox() {
                            ui.label(format!(
                                "#{} [{}] -> {:?}",
                                entry.entry_id, entry.document_key, entry.status
                            ));
                        }
                    }
                    DesktopInspectorTab::SyncTelemetry => {
                        ui.label("Target: Desktop High-Density Surface");
                        ui.label("Storage: OPFS / IndexedDB Local-First");
                    }
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Desktop Skill Lab Editor");
            ui.horizontal(|ui| {
                ui.label("Document Key:");
                ui.text_edit_singleline(&mut self.draft_key);
                ui.label("Title:");
                ui.text_edit_singleline(&mut self.draft_title);
            });
            ui.add(
                egui::TextEdit::multiline(&mut self.draft_body)
                    .desired_rows(10)
                    .desired_width(f32::INFINITY),
            );
            if ui.button("Commit Local Mutation (Outbox)").clicked() {
                let _ = self.engine.upsert_document(
                    self.draft_key.clone(),
                    self.draft_title.clone(),
                    self.draft_body.clone(),
                );
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desktop_workspace_renders_and_applies_dense_metrics() {
        let mut engine = OfflineStateEngine::new("desktop-node-01");
        engine
            .upsert_document("guidelines/arch", "Architecture", "Strict separation")
            .unwrap();

        let mut app = DesktopWorkspaceApp::new(engine);
        let ctx = Context::default();

        let _ = ctx.run(egui::RawInput::default(), |ctx| {
            app.render(ctx);
        });

        let style = ctx.style();
        assert_eq!(style.spacing.interact_size.y, DESKTOP_ROW_HEIGHT_PX);
        assert_eq!(app.engine().documents().len(), 1);
    }
}

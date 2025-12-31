use crate::circuit::VisualCircuit;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct QuirkApp {
    main_circuit: VisualCircuit,
}

impl Default for QuirkApp {
    fn default() -> Self {
        Self {
            main_circuit: VisualCircuit::default()
        }
    }
}

impl QuirkApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            Default::default()
        }
    }
}

impl eframe::App for QuirkApp {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::MenuBar::new().ui(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default()
            .frame(egui::Frame::default()
                .inner_margin(0.0)
                .fill(ctx.style().visuals.panel_fill))
            .show(ctx, |ui| {
                // TODO remove this scroll
                egui::ScrollArea::both().show(ui, |ui| {
                    ui.vertical(|ui| {
                        toolbox(ui);
                        grid(ui);
                    });
                });
            });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.columns_const(|[left, right]| {
                powered_by_egui_and_eframe(left);

                right.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    egui::warn_if_debug_build(ui);
                    ui.add(egui::github_link_file!(
                        "https://github.com/s5bug/quirk/",
                        "Source code."
                    ));
                });
            });
        });
    }
}

fn toolbox(ui: &mut egui::Ui) {
    // TODO make this a horizontal scroll
    let mut frame = egui::Frame::new();
    frame.fill = ui.visuals().extreme_bg_color;
    frame.inner_margin = ui.style().spacing.window_margin;
    frame.show(ui, |ui| {
        ui.heading("Toolbox");
        ui.take_available_width();
    });
}

fn grid(ui: &mut egui::Ui) {
    // TODO make this a both scroll
    let mut frame = egui::Frame::new();
    frame.inner_margin = ui.style().spacing.window_margin;
    frame.show(ui, |ui| {
        egui::Grid::new("main_circuit_grid").show(ui, |ui| {
            ui.label("|0>");
            ui.label("-");
            ui.end_row();
        });
    });
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

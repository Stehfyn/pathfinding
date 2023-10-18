use crate::{
    demo_panel::DemoPanel, demo_settings_panel::DemoSettingsPanel, top_panel::TopPanel, Panel,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Pathfinding {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)]
    top_panel: TopPanel,

    #[serde(skip)]
    demo_settings_panel: DemoSettingsPanel,

    #[serde(skip)]
    demo_panel: DemoPanel,
}

impl Default for Pathfinding {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            top_panel: TopPanel::default(),
            demo_settings_panel: DemoSettingsPanel::default(),
            demo_panel: DemoPanel::default(),
        }
    }
}

impl Pathfinding {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for Pathfinding {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        ctx.request_repaint();

        self.top_panel(ctx, _frame);
        self.demo_settings_panel(ctx, _frame);
        self.demo_panel(ctx, _frame);
    }
}

impl Pathfinding {
    fn top_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.top_panel.update(ctx, _frame);
    }
    fn demo_settings_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.demo_settings_panel.open = self.top_panel.is_demo_settings_open();
        self.demo_settings_panel.update(ctx, _frame);
    }
    fn demo_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.demo_panel.update(ctx, _frame);
    }
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

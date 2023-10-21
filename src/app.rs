use eframe::App;

use crate::{
    app_settings_panel::AppSettingsPanel, demo_panel::DemoPanel,
    demo_settings_panel::DemoSettingsPanel, entity_property_panel::EntityPropertyPanel,
    scene_hierarchy_panel::SceneHierarchyPanel, top_panel::TopPanel, Panel,
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

    #[serde(skip)]
    app_settings_panel: AppSettingsPanel,

    #[serde(skip)]
    scene_hierarchy_panel: SceneHierarchyPanel,

    #[serde(skip)]
    entity_property_panel: EntityPropertyPanel,
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
            app_settings_panel: AppSettingsPanel::default(),
            scene_hierarchy_panel: SceneHierarchyPanel::default(),
            entity_property_panel: EntityPropertyPanel::default(),
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
        self.app_settings_panel(ctx, _frame);
        self.scene_hierarchy_panel(ctx, _frame);
        self.entity_property_panel(ctx, _frame);
    }
}

impl Pathfinding {
    fn top_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.top_panel
            .set_font_scale(self.app_settings_panel.get_font_scale());
        self.top_panel.update(ctx, _frame);
    }
    fn demo_settings_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.demo_settings_panel.open = self.top_panel.is_demo_settings_open();
        self.demo_settings_panel
            .set_font_scale(self.app_settings_panel.get_font_scale());
        self.demo_settings_panel.update(ctx, _frame);
    }
    fn demo_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.demo_panel
            .set_env_settings(self.demo_settings_panel.get_env_settings());
        self.demo_panel.update(ctx, _frame);
    }
    fn app_settings_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.app_settings_panel.open = self.top_panel.is_app_settings_open();
        self.app_settings_panel.update(ctx, _frame);
        if self.app_settings_panel.is_logger_open() {
            egui::Window::new("Log").title_bar(false).show(ctx, |ui| {
                // draws the logger ui.
                egui_logger::minimal_logger_ui(ui, egui::Color32::BLACK);
            });
        }
    }
    fn scene_hierarchy_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.scene_hierarchy_panel.open = true;
        self.scene_hierarchy_panel.update(ctx, _frame);
    }
    fn entity_property_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.entity_property_panel.open = true;
        self.entity_property_panel.update(ctx, _frame);
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

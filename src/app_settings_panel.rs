use eframe::App;

use crate::fixed_demo_label;

use super::Panel;
use super::MAX_WRAP;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppSettings {
    pub app_settings_panel_dimensions: egui::Vec2,
    pub demo_settings_panel_dimensions: egui::Vec2,
    pub font_scale: f32,
    pub show_logger: bool,
    pub show_entity_delete_button: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            app_settings_panel_dimensions: egui::Vec2 { x: 400., y: 300. },
            demo_settings_panel_dimensions: egui::Vec2 { x: 400., y: 300. },
            font_scale: 1.0,
            show_logger: false,
            show_entity_delete_button: true,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AppSettingsPanel {
    pub open: bool,
    font_size: f32,
    app_settings: AppSettings,
    demo_label: String,
}

impl Default for AppSettingsPanel {
    fn default() -> Self {
        let fixed_demo_label = fixed_demo_label();
        Self {
            open: true,
            font_size: 20.,
            app_settings: AppSettings::default(),
            demo_label: fixed_demo_label,
        }
    }
}

impl Panel for AppSettingsPanel {
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut open = self.open;

        let dimensions = self.app_settings.app_settings_panel_dimensions;

        egui::Window::new("⚙ App Settings")
            .fixed_pos((
                ctx.screen_rect().max.x
                    - dimensions.x
                    - (ctx.style().spacing.window_margin.left
                        + ctx.style().spacing.window_margin.right
                        + 40.),
                ((ctx.screen_rect().max.y / 2.) - (dimensions.y / 2.)),
            ))
            .fixed_size(dimensions)
            .constrain(true)
            .title_bar(false)
            .open(&mut open)
            .show(ctx, |ui| {
                let mut style = (*ctx.style()).clone();
                if let Some(text_style) = style.text_styles.get_mut(&egui::TextStyle::Button) {
                    *text_style = egui::FontId::new(
                        self.font_size * self.app_settings.font_scale,
                        egui::FontFamily::Proportional,
                    );
                }
                ui.style_mut().text_styles = style.text_styles;

                self.window_header(ui);
                self.app_settings_ui(ui);
                self.egui_settings_ui(ui);
            });
        self.open = open;
    }
    #[allow(unused)]
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {}
}

impl AppSettingsPanel {
    pub fn get_font_scale(&self) -> f32 {
        self.app_settings.font_scale
    }
    pub fn is_logger_open(&self) -> bool {
        self.app_settings.show_logger
    }
}

impl AppSettingsPanel {
    fn window_header(&mut self, ui: &mut egui::Ui) {
        ui.scope(|ui| {
            ui.vertical_centered(|ui| {
                ui.heading(
                    egui::RichText::new("⚙ App Settings")
                        .size(self.font_size * self.app_settings.font_scale),
                );
            });
            ui.separator();
        });
    }
    fn app_settings_ui(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new(&self.demo_label)
            .default_open(true)
            .show(ui, |ui| {
                self.font_scale_slider(ui);
                self.app_logger_checkbox(ui);
                self.entity_delete_button_checkbox(ui);
            });
    }

    fn egui_settings_ui(&mut self, ui: &mut egui::Ui) {
        ui.scope(|ui| {
            egui::ScrollArea::new([true, true]).show(ui, |ui| {
                let c = ui.ctx().clone();
                c.settings_ui(ui);
            });
        });
    }
}

impl AppSettingsPanel {
    fn font_scale_slider(&mut self, ui: &mut egui::Ui) {
        ui.scope(|ui| {
            let scale = self.app_settings.font_scale;
            ui.add(
                egui::Slider::new(&mut self.app_settings.font_scale, 0.5..=1.5)
                    .text(egui::RichText::new("font scale").size(self.font_size * scale)),
            )
        });
    }

    fn entity_delete_button_checkbox(&mut self, ui: &mut egui::Ui) {
        ui.scope(|ui| {
            ui.checkbox(
                &mut self.app_settings.show_entity_delete_button,
                "Show Entity Delete in Hierarchy",
            );
        });
    }
    fn app_logger_checkbox(&mut self, ui: &mut egui::Ui) {
        ui.scope(|ui| {
            ui.checkbox(&mut self.app_settings.show_logger, "Show Logger");
        });
    }
}

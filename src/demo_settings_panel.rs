use std::ops::RangeInclusive;

use super::Panel;
use super::MAX_WRAP;

#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum Stage {
    Minimal,
    Office,
    Generated,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum Generated {
    N(usize),
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum Obstacle {
    Rectangular(egui::Rect),
    Circular(f32),
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct EnvironmentSettings {
    pub stage: Stage,
    pub n: Generated,
    pub obstacle: Obstacle,
    pub width: f32,
    pub height: f32,
    pub radius: f32,
}

impl Default for EnvironmentSettings {
    fn default() -> Self {
        Self {
            stage: Stage::Generated,
            n: Generated::N(20 as usize),
            obstacle: Obstacle::Rectangular(egui::Rect::from_x_y_ranges(
                RangeInclusive::new(0., 5.),
                RangeInclusive::new(0., 5.),
            )),
            width: 5.,
            height: 5.,
            radius: 2.5,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct DemoSettingsPanel {
    pub open: bool,
    #[serde(skip)]
    font_size: f32,
    #[serde(skip)]
    sep_width: f32,
    #[serde(skip)]
    height: f32,
    #[serde(skip)]
    widget_rects: Vec<egui::Rect>,
    #[serde(skip)]
    sub_header_rects: Vec<egui::Rect>,
    #[serde(skip)]
    label: String,
    #[serde(skip)]
    next_dimensions: egui::Vec2,
    #[serde(skip)]
    env_settings: EnvironmentSettings,
}

impl Default for DemoSettingsPanel {
    fn default() -> Self {
        Self {
            open: false,
            font_size: 20.0,
            sep_width: 0.0,
            height: 200.0,
            widget_rects: Vec::default(),
            sub_header_rects: Vec::default(),
            label: "🖧 Configure".to_owned(),
            next_dimensions: egui::vec2(400., 300.),
            env_settings: EnvironmentSettings::default(),
        }
    }
}

impl Panel for DemoSettingsPanel {
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let mut open = self.open;
        egui::Window::new("My Window")
            .fixed_size(self.next_dimensions)
            .title_bar(false)
            .open(&mut open)
            .show(ctx, |ui| {
                self.calc_sub_header_rects(ui);

                ui.scope(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(self.label.clone());
                    });
                });

                ui.separator();
                ui.style_mut().spacing.item_spacing.y = 0.;

                ui.horizontal(|ui| {
                    ui.scope(|ui| {
                        ui.style_mut().spacing.item_spacing.x = 100.;
                        ui.add_space(
                            ((ui.available_width() - self.calc_sub_header_widths(ui.style()))
                                / 2.0)
                                + (ui.style().spacing.item_spacing.x / 2.0),
                        );
                        ui.label(egui::RichText::new("Environment").size(20.));
                        ui.label(egui::RichText::new("Pathfinding").size(20.));
                    });
                });

                ui.horizontal(|ui| {
                    ui.scope(|ui| {
                        let og_x = ui.style().spacing.item_spacing.x;
                        ui.style_mut().spacing.item_spacing.x = 100.;
                        ui.add_space(
                            ((ui.available_width() - self.calc_sub_header_widths(ui.style()))
                                / 2.0)
                                + (ui.style().spacing.item_spacing.x / 2.0),
                        );
                        let mut n = self.env_settings.n;
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.radio_value(
                                    &mut self.env_settings.stage,
                                    Stage::Minimal,
                                    "Minimal",
                                );
                                ui.radio_value(
                                    &mut self.env_settings.stage,
                                    Stage::Office,
                                    "Office",
                                );
                                ui.radio_value(
                                    &mut self.env_settings.stage,
                                    Stage::Generated,
                                    "Generated",
                                );
                                ui.style_mut().spacing.item_spacing.x = og_x;
                                ui.scope(|ui| {
                                    ui.set_enabled(self.env_settings.stage == Stage::Generated);

                                    egui::ComboBox::from_label("N")
                                        .selected_text(format!("{n:?}"))
                                        .show_ui(ui, |ui| {
                                            ui.style_mut().wrap = Some(false);
                                            ui.set_min_width(60.0);
                                            ui.selectable_value(
                                                &mut n,
                                                Generated::N(20 as usize),
                                                "20",
                                            );
                                            ui.selectable_value(
                                                &mut n,
                                                Generated::N(30 as usize),
                                                "30",
                                            );
                                            ui.selectable_value(
                                                &mut n,
                                                Generated::N(100 as usize),
                                                "100",
                                            );
                                        });
                                    ui.radio_value(
                                        &mut self.env_settings.obstacle,
                                        Obstacle::Rectangular(egui::Rect::from_x_y_ranges(
                                            RangeInclusive::new(0., self.env_settings.width),
                                            RangeInclusive::new(0., self.env_settings.height),
                                        )),
                                        "Rectangular",
                                    );
                                    ui.radio_value(
                                        &mut self.env_settings.obstacle,
                                        Obstacle::Circular(self.env_settings.radius),
                                        "Circular",
                                    );
                                });
                            });
                        });
                        self.env_settings.n = n;
                        ui.group(|ui| {
                            ui.vertical(|ui| {
                                ui.radio(false, "A*");
                                ui.radio(false, "Waypoint Generation");
                                ui.radio(false, "Potential Fields");
                            });
                        });
                    });
                });
            });
        self.open = open;
    }
    #[allow(unused)]
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {}
}

impl DemoSettingsPanel {
    fn calc_sub_header_rects(&mut self, ui: &mut egui::Ui) {
        self.sub_header_rects.clear();

        self.sub_header_rects.push(
            ui.painter()
                .layout(
                    "Environment".to_owned(),
                    egui::FontId::new(self.font_size, egui::FontFamily::Proportional),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );

        self.sub_header_rects.push(
            ui.painter()
                .layout(
                    "Pathfinding".to_owned(),
                    egui::FontId::new(self.font_size, egui::FontFamily::Proportional),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );
    }

    #[allow(unused)]
    pub fn calc_sub_header_widths(&mut self, style: &egui::Style) -> f32 {
        let mut width = 0.;
        let item_spacing_x = style.spacing.item_spacing.x;
        let button_padding_x = style.spacing.button_padding.x;

        for r in self.sub_header_rects.iter() {
            width += r.width();
            width += button_padding_x * 2.;
            width += item_spacing_x;
        }

        width
    }

    fn calc_sub_header_height(&mut self) -> f32 {
        // Look at the tallest button
        self.sub_header_rects
            .iter()
            .map(|r| r.height())
            .fold(f32::NEG_INFINITY, |a, b| a.max(b))
    }
}



//use crate::ecs::component2::Component3;
use crate::ecs::{component::*};

use super::Panel;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct EntityPropertyPanel {
    pub open: bool,
    #[serde(skip)]
    font_size: f32,
    #[serde(skip)]
    font_scale: f32,
    #[serde(skip)]
    dimensions: egui::Vec2,
    //#[serde(skip)]
    //c: crate::ecs::component2::Component2,
}

impl Default for EntityPropertyPanel {
    fn default() -> Self {
        Self {
            open: false,
            font_size: 20.,
            font_scale: 1.,
            dimensions: egui::vec2(200., 400.),
        }
    }
}

impl Panel for EntityPropertyPanel {
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.open {
            egui::Window::new(
                egui::RichText::new("Inspector").size(self.font_size * self.font_scale),
            )
            .fixed_pos((ctx.screen_rect().width() - self.dimensions.x - 40., 40.))
            .fixed_size(self.dimensions)
            .collapsible(false)
            .show(ctx, |ui| {
                self.entity_property_ui(ui);
            });
        }
    }
    #[allow(unused)]
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {}
}

impl EntityPropertyPanel {
    pub fn entity_property_ui(&mut self, ui: &mut egui::Ui) {
        unsafe {
            ui.scope(|ui| {
                let mut style = (*ui.ctx().style()).clone();
                if let Some(text_style) = style.text_styles.get_mut(&egui::TextStyle::Button) {
                    *text_style = egui::FontId::new(
                        self.font_size * self.font_scale,
                        egui::FontFamily::Proportional,
                    );
                }
                if let Some(text_style) = style.text_styles.get_mut(&egui::TextStyle::Body) {
                    *text_style = egui::FontId::new(
                        self.font_size * self.font_scale,
                        egui::FontFamily::Proportional,
                    );
                }
                ui.style_mut().text_styles = style.text_styles;
                //egui::Resize::default().
                let _layout = egui::Layout::left_to_right(egui::Align::Center)
                    .with_cross_align(egui::Align::Center)
                    .with_cross_justify(false)
                    .with_main_wrap(true)
                    .with_main_align(egui::Align::Center);
                let selected = super::scene_hierarchy_panel::get_selected();
                for entt in selected.iter() {
                    if *entt != 0 {}
                }

                let mut selected_entities = crate::ecs::entity::get_selected_entities();
                for e in selected_entities.iter_mut() {
                    if e.get_id() != 0_usize {
                        let mut edata = e.data.get_ui_drawer();
                        edata(ui);

                        ui.separator();

                        for c in &mut e.components {
                            match c {
                                Component::Transform2(tc) => {
                                    //ui.vertical_centered(|ui| ui.label(tc.component_data.name.to_string()));
                                    ui.vertical_centered(|ui| {
                                        ui.label(
                                            egui::RichText::new("Transform")
                                                .size(self.font_size * self.font_scale * 0.8),
                                        )
                                    });
                                    let mut drawer = tc.get_mut().get_ui_drawer();
                                    drawer(ui);
                                }
                                Component::Color(cc) => {
                                    ui.vertical_centered(|ui| {
                                        ui.label(
                                            egui::RichText::new("Color")
                                                .size(self.font_size * self.font_scale * 0.8),
                                        )
                                    });

                                    let mut drawer = cc.get_mut().get_ui_drawer();
                                    drawer(ui);
                                }
                                Component::Mesh(mc) => {
                                    ui.vertical_centered(|ui| {
                                        ui.label(
                                            egui::RichText::new("Mesh")
                                                .size(self.font_size * self.font_scale * 0.8),
                                        )
                                    });

                                    let mut drawer = mc.get_mut().get_ui_drawer();
                                    drawer(ui);
                                }
                            }
                            ui.separator();
                        }
                    }
                }
            });
        }
    }
}

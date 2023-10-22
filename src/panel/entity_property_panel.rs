use super::Panel;

pub trait Component {
    fn update(&mut self);
}

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
    #[serde(skip)]
    c: crate::ecs::component2::Component2,
}

impl Default for EntityPropertyPanel {
    fn default() -> Self {
        Self {
            open: false,
            font_size: 20.,
            font_scale: 1.,
            dimensions: egui::vec2(200., 400.),
            c: crate::ecs::component2::Component2 {
                x: 32.,
                name: "example".to_string(),
            },
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
        let mut drawer = self.c.get_ui_drawer();
        // Somewhere in your rendering loop where egui Ui is available:
        drawer(ui);
        unsafe {
            let selected = super::scene_hierarchy_panel::get_selected();
            for entt in selected.iter() {
                if *entt != 0 {
                    ui.label(format!("Entity {}", *entt));
                }
            }
        }
    }
}

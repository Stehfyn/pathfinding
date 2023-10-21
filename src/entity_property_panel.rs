use super::Panel;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct EntityPropertyPanel {
    pub open: bool,
    font_size: f32,
}

impl Default for EntityPropertyPanel {
    fn default() -> Self {
        Self {
            open: false,
            font_size: 20.,
        }
    }
}

impl Panel for EntityPropertyPanel {
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {}
    #[allow(unused)]
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {}
}

#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod app_settings_panel;
mod demo_panel;
mod demo_settings_panel;
mod entity_property_panel;
mod pathfinding;
mod scene_hierarchy_panel;
mod top_panel;
pub use app::Pathfinding;
const MAX_WRAP: f32 = 1000.0;

pub trait Panel {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame);
}

pub fn fixed_demo_label() -> String {
    let demo_label = "🗺️Demo".to_string();
    let end_index = demo_label
        .char_indices()
        .nth(1)
        .map_or(demo_label.len(), |(i, _)| i);
    let mut sliced: String = demo_label[..end_index].to_string();
    sliced += " Demo";
    sliced
}

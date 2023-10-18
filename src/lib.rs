#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod demo_settings_panel;
mod demo_panel;
mod top_panel;
pub use app::Pathfinding;
const MAX_WRAP: f32 = 1000.0;

pub trait Panel {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame);
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame);
}

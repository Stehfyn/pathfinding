#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    //env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    egui_logger::init_with_max_level(log::LevelFilter::Debug).expect("Error initializing logger");
    let log_color_map = egui_logger::LogColorMap::new(
        egui::Color32::LIGHT_GREEN,
        egui::Color32::from_rgb(0, 0, 255),   // Blue
        egui::Color32::WHITE,                 // Green
        egui::Color32::from_rgb(255, 165, 0), // Orange
        egui::Color32::from_rgb(255, 0, 0),   // Red
    );

    egui_logger::set_log_color_map(log_color_map);

    let native_options = eframe::NativeOptions {
        initial_window_size: Some([400.0, 300.0].into()),
        min_window_size: Some([300.0, 220.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(pathfinding::Pathfinding::new(cc))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    //eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    egui_logger::init_with_max_level(log::LevelFilter::Debug).expect("Error initializing logger");
    let log_color_map = egui_logger::LogColorMap::new(
        egui::Color32::LIGHT_GREEN,
        egui::Color32::from_rgb(0, 0, 255),   // Blue
        egui::Color32::WHITE,                 // Green
        egui::Color32::from_rgb(255, 165, 0), // Orange
        egui::Color32::from_rgb(255, 0, 0),   // Red
    );

    egui_logger::set_log_color_map(log_color_map);

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(pathfinding::Pathfinding::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}

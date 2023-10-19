use super::Panel;
use super::MAX_WRAP;
use egui::RichText;
use rand::Rng;
use std::f64::consts::TAU;
use std::ops::RangeInclusive;

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

#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize)]
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

#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct CircleParams {
    center_x: f64,
    center_y: f64,
    radius_x: f64,
    radius_y: f64,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct RectParams {
    center_x: f64,
    center_y: f64,
    width: f64,
    height: f64,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum ShapeParams {
    Circle(CircleParams),
    Rectangle(RectParams),
}

#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
pub struct DemoPanel {
    pub open: bool,
    marker_size: f32,
    grid: egui::Rect,
    env_settings: EnvironmentSettings,
    cursor_x: i32,
    cursor_y: i32,

    base_points: Vec<[f64; 2]>,
    waypoint_points: Vec<[f64; 2]>,
    path_points: Vec<[f64; 2]>,
    search_points: Vec<[f64; 2]>,
    selected_points: Vec<[f64; 2]>,
    hovered_points: Vec<[f64; 2]>,
    generate: bool,
    obstacles: Vec<ShapeParams>,
}

impl Default for DemoPanel {
    fn default() -> Self {
        Self {
            open: true,
            marker_size: 0.0,
            grid: egui::Rect::from_min_max(
                egui::Pos2 { x: 0., y: 0. },
                egui::Pos2 { x: 100., y: 100. },
            ),
            env_settings: EnvironmentSettings::default(),

            cursor_x: i32::MAX,
            cursor_y: i32::MAX,

            base_points: Vec::new(),
            waypoint_points: Vec::new(),
            path_points: Vec::new(),
            search_points: Vec::new(),
            selected_points: Vec::new(),
            hovered_points: Vec::new(),
            generate: false,
            obstacles: Vec::new(),
        }
    }
}

impl Panel for DemoPanel {
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::Window::new("demo_panel")
            .min_height(100.)
            .min_height(100.)
            .title_bar(false)
            .show(ctx, |ui| {
                if ui
                    .button(egui::RichText::new("generate").size(20.))
                    .clicked()
                {
                    self.generate = true;
                }
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = egui::Painter::new(
                ui.ctx().clone(),
                ui.layer_id(),
                ui.available_rect_before_wrap(),
            );
            self.paint_grid(ui, &painter);
            // Make sure we allocate what we used (everything)
            ui.expand_to_include_rect(painter.clip_rect());

            ui.style_mut().spacing.item_spacing.x = 0.;
        });
        self.generate = false;
    }
    #[allow(unused)]
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {}
}

impl DemoPanel {
    pub fn set_env_settings(&mut self, new_settings: EnvironmentSettings) {
        self.env_settings = new_settings;
    }
}

impl DemoPanel {
    fn paint_grid(&mut self, ui: &mut egui::Ui, painter: &egui::Painter) {
        let rect = painter.clip_rect();

        let mut markers = self.calc_markers(ui);
        let hovered_markers = markers.remove(0);
        let base_markers = markers.remove(0);
        let plot = egui_plot::Plot::new("navmesh")
            .legend(egui_plot::Legend::default().position(egui_plot::Corner::RightBottom))
            .show_x(false)
            .show_y(false)
            .y_axis_width(2)
            .allow_zoom(false)
            .auto_bounds_x()
            .auto_bounds_y()
            .include_x(100)
            .include_y(100)
            .include_x(0)
            .include_y(0)
            .data_aspect(1.0);

        plot.show(ui, |plot_ui| {
            let mut x: i32 = 0;
            let mut y: i32 = 0;

            if let Some(point) = plot_ui.pointer_coordinate() {
                x = point.x as i32;
                y = point.y as i32;

                self.cursor_x = x;
                self.cursor_y = y;
            }

            //plot_ui.points(base_markers);
            plot_ui.points(hovered_markers);

            if self.generate {
                self.obstacles.clear();
                match self.env_settings.n {
                    Generated::N(n) => {
                        for _ in 0..n {
                            match self.env_settings.obstacle {
                                Obstacle::Circular(r) => {
                                    let center_x = rand::thread_rng().gen_range(0..100) as f64;
                                    let center_y = rand::thread_rng().gen_range(0..100) as f64;
                                    let radius_x = r;
                                    let radius_y = r;

                                    let circle_params = CircleParams {
                                        center_x: center_x as f64,
                                        center_y: center_y as f64,
                                        radius_x: radius_x as f64,
                                        radius_y: radius_y as f64,
                                    };

                                    self.obstacles.push(ShapeParams::Circle(circle_params));
                                }
                                Obstacle::Rectangular(r) => {
                                    let center_x = rand::thread_rng().gen_range(0..100) as f64;
                                    let center_y = rand::thread_rng().gen_range(0..100) as f64;
                                    let width = r.width();
                                    let height = r.height();

                                    let rect_params = RectParams {
                                        center_x: center_x,
                                        center_y: center_y,
                                        width: width as f64,
                                        height: height as f64,
                                    };

                                    self.obstacles.push(ShapeParams::Rectangle(rect_params));
                                }
                            }
                        }
                    }
                }
            }

            for obs in self.obstacles.iter() {
                match obs {
                    ShapeParams::Circle(cp) => {
                        let circle = self.create_circle(cp.center_x, cp.center_y, cp.radius_y);
                        plot_ui.polygon(circle);
                    }
                    ShapeParams::Rectangle(rp) => {
                        let rect =
                            self.create_rectangle(rp.center_x, rp.center_y, rp.width, rp.height);
                        for line in rect {
                            plot_ui.line(line);
                        }
                    }
                }
            }

            let dist = plot_ui
                .screen_from_plot(egui_plot::PlotPoint::new(0.0, 0.0))
                .distance(plot_ui.screen_from_plot(egui_plot::PlotPoint::new(100.0, 0.0)));

            // 1.8 for overlap
            self.marker_size = dist / (100. * 2.2);
        });
    }
}

impl DemoPanel {
    fn calc_markers(&mut self, ui: &egui::Ui) -> Vec<egui_plot::Points> {
        self.base_points.clear();
        self.waypoint_points.clear();
        self.path_points.clear();
        self.search_points.clear();
        self.selected_points.clear();
        self.hovered_points.clear();

        let startx = self.grid.min.x as i32;
        let starty = self.grid.min.y as i32;
        let endx = startx + self.grid.width() as i32;
        let endy = starty + self.grid.height() as i32;

        for i in startx..endx {
            for j in starty..endy {
                let point = [(i as f64 + 0.5f64), (j as f64 + 0.5f64)];
                if (j == self.cursor_y) && (i == self.cursor_x) {
                    self.hovered_points.push(point);
                } else {
                    self.base_points.push(point);
                }
            }
        }

        let base_color = if ui.visuals().dark_mode {
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 25)
        } else {
            egui::Color32::from_rgba_unmultiplied(25, 25, 25, 25)
        };

        let hovered_markers = egui_plot::Points::new(self.hovered_points.clone())
            .filled(true)
            .radius(self.marker_size)
            .highlight(true)
            .color(egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255)) // Red color
            .shape(egui_plot::MarkerShape::Square);

        let base_markers = egui_plot::Points::new(self.base_points.clone())
            .filled(true)
            .radius(self.marker_size)
            .highlight(true)
            .color(base_color)
            .shape(egui_plot::MarkerShape::Square);

        vec![hovered_markers, base_markers]
    }

    fn create_rectangle(&self, x: f64, y: f64, width: f64, height: f64) -> Vec<egui_plot::Line> {
        let top_left = [x, y];
        let top_right = [x + width, y];
        let bottom_left = [x, y + height];
        let bottom_right = [x + width, y + height];
        // left line

        vec![
            egui_plot::Line::new(egui_plot::PlotPoints::new(vec![
                top_left,
                top_right,
                bottom_right,
                bottom_left,
                top_left,
            ]))
            .fill(top_left[1] as f32), // top line
        ]
    }

    fn create_circle(&self, cx: f64, cy: f64, r: f64) -> egui_plot::Polygon {
        egui_plot::Polygon::new(egui_plot::PlotPoints::from_parametric_callback(
            |t| (r * t.sin() + cx, r * t.cos() + cy),
            0.0..TAU,
            100,
        ))
    }

    fn create_ellipse(&self, cx: f64, cy: f64, rx: f64, ry: f64) -> egui_plot::Polygon {
        egui_plot::Polygon::new(egui_plot::PlotPoints::from_parametric_callback(
            |t| (rx * t.sin() + cx, ry * t.cos() + cy),
            0.0..TAU,
            100,
        ))
    }

    fn generate_obstacles(&mut self) {}
}

use super::Panel;
use super::MAX_WRAP;
use egui::RichText;
use rand::Rng;
use std::f64::consts::TAU;
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum Stage {
    AStar,
    Office,
    Generated,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum Generated {
    N(usize),
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub enum Obstacle {
    Rectangular,
    Circular,
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct AStarStageSettings {
    pub red_notch_size: f32,
    pub blue_notch_size: f32,
}

impl Default for AStarStageSettings {
    fn default() -> Self {
        Self {
            red_notch_size: 3.,
            blue_notch_size: 2.,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct EnvironmentSettings {
    pub stage: Stage,
    pub n: Generated,
    pub obstacle: Obstacle,

    pub a_star_stage_settings: AStarStageSettings,

    pub rect_side_min: f32,
    pub rect_side_max: f32,
    pub circle_radius_min: f32,
    pub circle_radius_max: f32,
}

impl Default for EnvironmentSettings {
    fn default() -> Self {
        Self {
            stage: Stage::Generated,
            n: Generated::N(20 as usize),
            obstacle: Obstacle::Rectangular,
            a_star_stage_settings: AStarStageSettings::default(),
            rect_side_min: 4.,
            rect_side_max: 5.,
            circle_radius_min: 2.,
            circle_radius_max: 3.,
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
    stretch: bool,
    first_frame: bool,
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
            first_frame: true,
            stretch: true,
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

            if !self.first_frame && self.stretch {
                self.stretch_grid_x_boundaries(plot_ui);
                self.stretch = false;
            }

            if self.first_frame {
                self.first_frame = false;
            }

            //draw grid
            self.grid_boundaries(plot_ui);

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
                                Obstacle::Circular => {
                                    let center_x = rand::thread_rng().gen_range(
                                        (self.grid.min.x as i32)..(self.grid.max.x as i32),
                                    ) as f64;
                                    let center_y = rand::thread_rng().gen_range(0..100) as f64;
                                    let radius_x = rand::thread_rng().gen_range(
                                        self.env_settings.circle_radius_min
                                            ..self.env_settings.circle_radius_max,
                                    ) as f64;
                                    let radius_y = radius_x;

                                    let circle_params = CircleParams {
                                        center_x: center_x as f64,
                                        center_y: center_y as f64,
                                        radius_x: radius_x as f64,
                                        radius_y: radius_y as f64,
                                    };

                                    self.obstacles.push(ShapeParams::Circle(circle_params));
                                }
                                Obstacle::Rectangular => {
                                    let center_x = rand::thread_rng().gen_range(
                                        (self.grid.min.x as i32)..(self.grid.max.x as i32),
                                    ) as f64;
                                    let center_y = rand::thread_rng().gen_range(0..100) as f64;
                                    let width = rand::thread_rng().gen_range(
                                        self.env_settings.rect_side_min
                                            ..self.env_settings.rect_side_max,
                                    ) as f64;
                                    let height = rand::thread_rng().gen_range(
                                        self.env_settings.rect_side_min
                                            ..self.env_settings.rect_side_max,
                                    ) as f64;

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

            if !(self.env_settings.stage == Stage::Office)
                && !(self.env_settings.stage == Stage::AStar)
            {
                for obs in self.obstacles.iter() {
                    match obs {
                        ShapeParams::Circle(cp) => {
                            let circle = self.create_circle(cp.center_x, cp.center_y, cp.radius_y);
                            plot_ui.polygon(circle);
                        }
                        ShapeParams::Rectangle(rp) => {
                            let rect = self.create_rectangle(
                                rp.center_x,
                                rp.center_y,
                                rp.width,
                                rp.height,
                            );
                            for line in rect {
                                plot_ui.line(line);
                            }
                        }
                    }
                }
            } else if self.env_settings.stage == Stage::AStar {
                self.a_star_stage(plot_ui);
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
    fn a_star_stage(&mut self, plot_ui: &mut egui_plot::PlotUi) {
        // red
        let p1r = [30f64, 41f64];
        let p2r = [39f64, 41f64];
        let p3r = [41f64, 49f64];
        let p4r = [41f64, 41f64];
        let p5r = [50f64, 41f64];
        let p6r = [50f64, 40f64];
        let p7r = [41f64, 40f64];
        let p8r = [40f64, 37f64];
        let p9r = [39f64, 37f64];
        let p10r = [39f64, 40f64];
        let p11r = [30f64, 40f64];
        // left line

        plot_ui.line(
            egui_plot::Line::new(egui_plot::PlotPoints::new(vec![
                p1r, p2r, p3r, p4r, p5r, p6r, p7r, p8r, p9r, p10r, p11r, p1r,
            ]))
            .color(egui::Color32::from_rgba_unmultiplied(255, 0, 0, 125))
            .fill(40.),
        );

        // blue
        let p1b = [50f64, 49f64];
        let p2b = [41f64, 49f64];
        let p3b = [39f64, 41f64];
        let p4b = [39f64, 49f64];
        let p5b = [30f64, 49f64];
        let p6b = [30f64, 50f64];
        let p7b = [50f64, 50f64];
        // left line

        plot_ui.line(
            egui_plot::Line::new(egui_plot::PlotPoints::new(vec![
                p1b, p2b, p3b, p4b, p5b, p6b, p7b, p1b,
            ]))
            .color(egui::Color32::from_rgba_unmultiplied(0, 0, 255, 125))
            .fill(50.),
        );
    }
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

    fn stretch_grid_x_boundaries(&mut self, plot_ui: &mut egui_plot::PlotUi) {
        self.grid.min.x = plot_ui.plot_bounds().min()[0] as f32;
        self.grid.max.x = plot_ui.plot_bounds().max()[0] as f32;
    }

    fn grid_boundaries(&self, plot_ui: &mut egui_plot::PlotUi) {
        let top_left = [self.grid.left() as f64, self.grid.top() as f64];
        let top_right = [self.grid.right() as f64, self.grid.top() as f64];
        let bottom_left = [self.grid.left() as f64, self.grid.bottom() as f64];
        let bottom_right = [self.grid.right() as f64, self.grid.bottom() as f64];

        plot_ui.line(
            egui_plot::Line::new(egui_plot::PlotPoints::new(vec![
                top_left,
                top_right,
                bottom_right,
                bottom_left,
                top_left,
            ]))
            .width(2.)
            .color(egui::Color32::from_rgba_unmultiplied(125, 125, 125, 125))
            .style(egui_plot::LineStyle::Dashed { length: 10. }),
        );
    }

    fn generate_obstacles(&mut self) {}
}

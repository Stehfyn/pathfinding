use super::scene_hierarchy_panel::get_selected;
use super::Panel;
use crate::ecs::entity::get_entity_from_id;
use crate::ecs::{component::*, entity::ENTITY_MANAGER};

use crate::ecs::pos2::{self, Pos2};
use crate::pathfinding::NavMesh;
use poll_promise::Promise;
use rand::Rng;
use std::collections::HashMap;
use std::collections::HashSet;
use std::f64::consts::TAU;

struct PathPromise(Option<Promise<Option<Vec<Pos2>>>>);
impl std::fmt::Debug for PathPromise {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Your custom logic here
        write!(f, "DebuggablePromise(...)")
    }
}
impl PartialEq for PathPromise {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
impl Clone for PathPromise {
    fn clone(&self) -> Self {
        Self(Option::None)
    }
}
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

//#[derive(Debug, PartialEq, Clone, serde::Deserialize, serde::Serialize)]
#[derive(Debug, PartialEq, Clone)]
pub struct DemoPanel {
    pub open: bool,
    marker_size: f32,
    grid: egui::Rect,
    env_settings: EnvironmentSettings,
    cursor_x: f64,
    cursor_y: f64,

    base_points: Vec<[f64; 2]>,
    waypoint_points: Vec<[f64; 2]>,
    path_points: Vec<[f64; 2]>,
    search_points: Vec<[f64; 2]>,
    selected_points: Vec<[f64; 2]>,
    hovered_points: Vec<[f64; 2]>,
    generate: bool,
    obstacles: Vec<ShapeParams>,
    space_lut: HashMap<(i64, i64), bool>,
    stretch: bool,
    first_frame: bool,

    navmesh: NavMesh,

    start: Pos2,
    end: Pos2,

    path: Vec<Pos2>,

    path_map: HashMap<usize, PathPromise>,
    current_paths: HashMap<usize, Vec<Pos2>>,

    pub is_waypoint: bool,
    timer: f32,

    queued_points: Vec<Pos2>,
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

            cursor_x: f64::MAX,
            cursor_y: f64::MAX,

            base_points: Vec::new(),
            waypoint_points: Vec::new(),
            path_points: Vec::new(),
            search_points: Vec::new(),
            selected_points: Vec::new(),
            hovered_points: Vec::new(),
            generate: false,
            obstacles: Vec::new(),
            space_lut: HashMap::default(),
            first_frame: true,
            stretch: false,

            navmesh: NavMesh::default(),
            start: Pos2::default(),
            end: Pos2::default(),
            path: Vec::default(),
            path_map: HashMap::default(),
            current_paths: HashMap::default(),
            is_waypoint: true,
            timer: 0.5,
            queued_points: Vec::default(),
        }
    }
}

impl Panel for DemoPanel {
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = egui::Painter::new(
                ui.ctx().clone(),
                ui.layer_id(),
                ui.available_rect_before_wrap(),
            );
            self.navmesh
                .set_grid_boundaries(Pos2::from_min(&self.grid), Pos2::from_max(&self.grid));
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
        if (self.env_settings.stage != Stage::AStar) && (new_settings.stage == Stage::AStar) {
            self.swap_to_a_star_stage();
        }
        self.env_settings = new_settings;
    }
}

impl DemoPanel {
    fn paint_grid(&mut self, ui: &mut egui::Ui, painter: &egui::Painter) {
        let _rect = painter.clip_rect();

        let mut markers = self.update_markers(ui);
        let hovered_markers = markers.remove(0);
        let base_markers = markers.remove(0);
        let path_markers = markers.remove(0);

        let _cursor = egui::CursorIcon::Default;

        let _r = ui.scope(|ui| {
            let plot = egui_plot::Plot::new("navmesh")
                .legend(egui_plot::Legend::default().position(egui_plot::Corner::RightBottom))
                .show_x(false)
                .show_y(false)
                .y_axis_width(2)
                .allow_zoom(false)
                .allow_boxed_zoom(true)
                .auto_bounds_x()
                .auto_bounds_y()
                .include_x(100)
                .include_y(100)
                .include_x(0)
                .include_y(0)
                .data_aspect(1.0);

            plot.show(ui, |plot_ui| {
                let (x, y) = self.update_cursor_pos(plot_ui);
                let _xy = egui::Pos2::new(x as f32, y as f32);

                if !self.first_frame && self.stretch {
                    self.stretch_grid_x_boundaries(plot_ui);
                    self.stretch = false;
                }

                self.draw_grid_boundaries(plot_ui);

                let show_obst_bounds = false;
                if show_obst_bounds {
                    if self.env_settings.stage == Stage::Generated {
                        plot_ui.points(base_markers);
                    }
                }

                self.timer -= plot_ui.ctx().input(|r| r.stable_dt);
                if self.timer <= 0.0 {
                    //advance entts
                    self.move_entities();
                    self.timer = 0.05;
                }
                if self.is_waypoint {
                    plot_ui.points(path_markers);
                } else {
                    // move entts
                }

                if self.generate {
                    self.generate_obstacles();
                }

                self.draw_stage(plot_ui);

                self.draw_entities(plot_ui);

                plot_ui.points(hovered_markers);

                self.update_marker_size(plot_ui);

                unsafe {
                    crate::ecs::entity::propagate_entity_changes();
                }
            });
        });

        if self.first_frame {
            self.first_frame = false;
        }
    }
}

fn fill_lut_with_rectangle(
    lut: &mut HashMap<(i64, i64), bool>,
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) {
    let min_ix = x.floor() as i64;
    let max_ix = (x + width).floor() as i64;
    let min_iy = y.floor() as i64;
    let max_iy = (y + height).floor() as i64;

    for ix in min_ix..=max_ix {
        for iy in min_iy..=max_iy {
            lut.insert((ix, iy), true);
        }
    }
}

fn fill_lut_with_circle(lut: &mut HashMap<(i64, i64), bool>, cx: f64, cy: f64, r: f64) {
    let min_x = (cx - r).floor() as i64;
    let max_x = (cx + r).ceil() as i64;
    let min_y = (cy - r).floor() as i64;
    let max_y = (cy + r).ceil() as i64;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if is_inside_circle(x, y, cx, cy, r) {
                lut.insert((x, y), true);
            }
        }
    }
}

fn is_inside_circle(x: i64, y: i64, cx: f64, cy: f64, r: f64) -> bool {
    let corners = [
        (x as f64, y as f64),
        (x as f64 + 1.0, y as f64),
        (x as f64, y as f64 + 1.0),
        (x as f64 + 1.0, y as f64 + 1.0),
    ];

    corners.iter().any(|&(px, py)| {
        let dx = px - cx;
        let dy = py - cy;
        dx * dx + dy * dy <= r * r
    })
}

impl DemoPanel {
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

    fn generate_obstacles(&mut self) {
        self.obstacles.clear();
        self.space_lut.clear();

        match self.env_settings.n {
            Generated::N(n) => {
                for _ in 0..n {
                    match self.env_settings.obstacle {
                        Obstacle::Circular => {
                            let center_x = rand::thread_rng()
                                .gen_range((self.grid.min.x as i32)..(self.grid.max.x as i32))
                                as f64;
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
                            fill_lut_with_circle(&mut self.space_lut, center_x, center_y, radius_x);
                            self.obstacles.push(ShapeParams::Circle(circle_params));
                        }
                        Obstacle::Rectangular => {
                            let center_x = rand::thread_rng()
                                .gen_range((self.grid.min.x as i32)..(self.grid.max.x as i32))
                                as f64;
                            let center_y = rand::thread_rng().gen_range(0..100) as f64;
                            let width = rand::thread_rng().gen_range(
                                self.env_settings.rect_side_min..self.env_settings.rect_side_max,
                            ) as f64;
                            let height = rand::thread_rng().gen_range(
                                self.env_settings.rect_side_min..self.env_settings.rect_side_max,
                            ) as f64;

                            let rect_params = RectParams {
                                center_x: center_x,
                                center_y: center_y,
                                width: width as f64,
                                height: height as f64,
                            };
                            fill_lut_with_rectangle(
                                &mut self.space_lut,
                                center_x,
                                center_y,
                                width,
                                height,
                            );
                            self.obstacles.push(ShapeParams::Rectangle(rect_params));
                        }
                    }
                }
            }
        }
        self.navmesh.set_space_lut(self.space_lut.clone());
    }
}

impl DemoPanel {
    fn draw_grid_boundaries(&self, plot_ui: &mut egui_plot::PlotUi) {
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

    fn draw_stage(&mut self, plot_ui: &mut egui_plot::PlotUi) {
        if self.env_settings.stage == Stage::Generated {
            self.generated_stage(plot_ui);
        } else if self.env_settings.stage == Stage::AStar {
            self.a_star_stage(plot_ui);
        }
    }

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

    fn generated_stage(&mut self, plot_ui: &mut egui_plot::PlotUi) {
        let force_col = egui::Color32::from_rgba_unmultiplied(255, 0, 165, 10);
        for obs in self.obstacles.iter() {
            match obs {
                ShapeParams::Circle(cp) => {
                    let circle = self
                        .create_circle(cp.center_x, cp.center_y, cp.radius_y)
                        .fill_color(force_col);
                    plot_ui.polygon(circle);
                }
                ShapeParams::Rectangle(rp) => {
                    let rect = self.create_rectangle(rp.center_x, rp.center_y, rp.width, rp.height);
                    for line in rect {
                        plot_ui.line(line.color(force_col));
                    }
                }
            }
        }
    }

    fn move_entities(&mut self) {
        unsafe {
            for (id, path) in self.current_paths.iter_mut() {
                let mut entt_opt = ENTITY_MANAGER.get_mut(id);
                if let Some(entt) = entt_opt {
                    for c in entt.components.iter_mut() {
                        if let Component::Transform2(tc) = c {
                            if let Some(pos) = path.first() {
                                tc.get_mut().pos = *pos;
                                path.remove(0);
                            }
                        }
                    }
                }
            }
        }
    }

    fn draw_entities(&mut self, plot_ui: &mut egui_plot::PlotUi) {
        unsafe {
            let entities = ENTITY_MANAGER.iter();
            let selected = get_selected();
            for (id, e) in entities {
                if *id != 0_usize {
                    let mut pos = pos2::Pos2::default();
                    let mut points_to_draw: Vec<[f64; 2]> = Vec::new();
                    let mut col = egui::Color32::default();
                    for c in &e.components {
                        match c {
                            Component::Transform2(tc) => {
                                pos = tc.get().pos;
                                points_to_draw.push([
                                    tc.get().pos.x as f64 + 0.5f64,
                                    tc.get().pos.y as f64 + 0.5f64,
                                ]);
                            }
                            Component::Color(cc) => {
                                col = cc.get().col;
                            }
                            Component::Mesh(_mc) => {}
                        }
                    }
                    let markers = egui_plot::Points::new(points_to_draw)
                        .filled(true)
                        .radius(self.marker_size * 1.2)
                        .highlight(true)
                        .color(col) // Red color
                        .shape(egui_plot::MarkerShape::Square);

                    plot_ui.points(markers);

                    if selected.contains(id) {
                        plot_ui.polygon(
                            self.create_circle(pos.x as f64 + 0.5f64, pos.y as f64 + 0.5f64, 3f64)
                                .name(e.data.name.clone()),
                        );
                    }
                }
            }
        }
    }
}

impl DemoPanel {
    fn update_markers(&mut self, ui: &egui::Ui) -> Vec<egui_plot::Points> {
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

        for ((x, y), covered) in &self.space_lut {
            if *covered {
                self.base_points
                    .push([(*x as f64).floor() + 0.5f64, (*y as f64).floor() + 0.5f64]);
            }
        }

        let mut unique_positions = HashSet::new();

        for path in self.current_paths.values() {
            for pos in path {
                if unique_positions.insert(*pos) {
                    self.path_points
                        .push([pos.x as f64 + 0.5, pos.y as f64 + 0.5]);
                }
            }
        }

        let x_range = startx..endx;
        let y_range = starty..endy;
        if x_range.contains(&(self.cursor_x.floor() as i32))
            && y_range.contains(&(self.cursor_y.floor() as i32))
        {
            self.hovered_points
                .push([self.cursor_x as f64, self.cursor_y as f64]);
        }

        let base_color = if ui.visuals().dark_mode {
            egui::Color32::from_rgba_unmultiplied(255, 255, 255, 25)
        } else {
            egui::Color32::from_rgba_unmultiplied(25, 25, 25, 25)
        };

        let path_color = if ui.visuals().dark_mode {
            egui::Color32::from_rgba_unmultiplied(25, 255, 25, 25)
        } else {
            egui::Color32::from_rgba_unmultiplied(25, 255, 25, 25)
        };

        let hovered_markers = egui_plot::Points::new(self.hovered_points.clone())
            .filled(true)
            .radius(self.marker_size * 0.8)
            .highlight(true)
            .color(egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255)) // Red color
            .shape(egui_plot::MarkerShape::Square);

        let base_markers = egui_plot::Points::new(self.base_points.clone())
            .filled(true)
            .radius(self.marker_size)
            .highlight(true)
            .color(base_color)
            .shape(egui_plot::MarkerShape::Square);

        let path_markers = egui_plot::Points::new(self.path_points.clone())
            .filled(true)
            .radius(self.marker_size)
            .highlight(true)
            .color(path_color)
            .shape(egui_plot::MarkerShape::Square);

        vec![hovered_markers, base_markers, path_markers]
    }

    fn stretch_grid_x_boundaries(&mut self, plot_ui: &mut egui_plot::PlotUi) {
        self.grid.min.x = plot_ui.plot_bounds().min()[0] as f32;
        self.grid.max.x = plot_ui.plot_bounds().max()[0] as f32;
    }

    fn update_cursor_pos(&mut self, plot_ui: &egui_plot::PlotUi) -> (f64, f64) {
        static mut PATH_PROMISE: Option<Promise<Option<Vec<Pos2>>>> = None;

        let mut x = f64::MIN;
        let mut y = f64::MIN;
        if let Some(point) = plot_ui.pointer_coordinate() {
            x = (point.x.floor() + 0.5) as f64;
            y = (point.y.floor() + 0.5) as f64;

            self.cursor_x = x;
            self.cursor_y = y;

            plot_ui.ctx().input(|ui| {
                if ui.pointer.button_clicked(egui::PointerButton::Middle) {
                    self.queued_points.push(pos2::Pos2 {
                        x: x as i64,
                        y: y as i64,
                    });
                }
            });

            plot_ui.ctx().input(|ui| unsafe {
                if ui.pointer.primary_clicked() {
                    let entts = crate::ecs::entity::get_entities_from_xy(x, y);
                    if entts.len() > 0 {
                        if !ui.raw.modifiers.ctrl {
                            crate::panel::scene_hierarchy_panel::unselect_all();
                        }

                        for e in entts.iter() {
                            crate::panel::scene_hierarchy_panel::select((*e).get_id());
                        }
                    } else {
                        self.navigate(x, y, ui);
                    }
                }
            });

            unsafe {
                let selected = get_selected();
                for s in selected.iter() {
                    if let Some(path_promise) = self.path_map.get_mut(s) {
                        if let Some(promise) = &mut path_promise.0 {
                            if let Some(path_result) = promise.ready_mut() {
                                if let Some(path) = path_result.take() {
                                    self.current_paths.insert(*s, path);
                                    self.path_map.remove(s);
                                } else {
                                    self.current_paths.remove(s);
                                    self.path_map.remove(s);
                                }
                                PATH_PROMISE = None;
                            }
                        }
                    } else {
                        // Handle the case where the key is not found in path_map if needed.
                    }
                }
            }
        } else {
        }
        (x, y)
    }

    fn update_marker_size(&mut self, plot_ui: &egui_plot::PlotUi) {
        let dist = plot_ui
            .screen_from_plot(egui_plot::PlotPoint::new(0.0, 0.0))
            .distance(plot_ui.screen_from_plot(egui_plot::PlotPoint::new(100.0, 0.0)));

        // 1.8 for overlap
        self.marker_size = dist / (100. * 2.2);
    }
}

impl DemoPanel {
    fn navigate(&mut self, x: f64, y: f64, _ui: &egui::InputState) {
        self.start.x = x as i64;
        self.start.y = y as i64;

        unsafe {
            let selected = get_selected();
            if selected.len() > 0 {
                self.queued_points.push(self.start);
            }

            for s in selected.iter() {
                if let Some(path_promise) = self.path_map.get_mut(s) {
                    // handle the Option
                    if path_promise.0.is_none() {
                        // check the inner Option of PathPromise
                        let e = get_entity_from_id(*s);
                        let mut pos = pos2::Pos2::default();
                        for c in e.components.iter() {
                            match c {
                                Component::Transform2(tc) => {
                                    pos = tc.get().pos;
                                }
                                _ => {}
                            }
                        }

                        path_promise.0 = self.navmesh.async_a_star(pos, self.start);

                        log::info!(
                            "{} ({}, {}) wants to go to ({}, {})",
                            s,
                            pos.x,
                            pos.y,
                            self.start.x,
                            self.start.y
                        );
                    }
                } else {
                    let e = get_entity_from_id(*s);
                    let mut pos = pos2::Pos2::default();
                    for c in e.components.iter() {
                        match c {
                            Component::Transform2(tc) => {
                                pos = tc.get().pos;
                            }
                            _ => {}
                        }
                    }

                    log::info!("{}", self.queued_points.len());
                    let some_path_promise = self
                        .navmesh
                        .async_waypointed_a_star(pos, self.queued_points.clone());

                    log::info!(
                        "{} ({}, {}) wants to go to ({}, {})",
                        s,
                        pos.x,
                        pos.y,
                        self.start.x,
                        self.start.y
                    );
                    self.path_map
                        .insert(e.get_id(), PathPromise(some_path_promise));
                }
            }
            self.queued_points.clear();
        }
    }
}

impl DemoPanel {
    pub fn generate(&mut self) {
        self.generate = true;
    }
    pub fn swap_to_a_star_stage(&mut self) {
        self.space_lut.clear();
        fill_lut_with_rectangle(&mut self.space_lut, 39., 37., 1., 12.); // mid
        fill_lut_with_rectangle(&mut self.space_lut, 30., 40., 19., 0.); // bot
        fill_lut_with_rectangle(&mut self.space_lut, 30., 49., 19., 0.); // top
        self.navmesh.set_space_lut(self.space_lut.clone());
    }
}

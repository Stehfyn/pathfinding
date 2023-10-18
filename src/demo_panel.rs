use super::Panel;
use super::MAX_WRAP;
use std::f64::consts::TAU;
use std::ops::RangeInclusive;
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct DemoPanel {
    pub open: bool,
    marker_size: f32,
    x: i32,
    y: i32,
}

impl Default for DemoPanel {
    fn default() -> Self {
        Self {
            open: true,
            marker_size: 0.0,
            x: -1,
            y: -1,
        }
    }
}
fn remap(value: f64, from_range: RangeInclusive<f64>, to_range: RangeInclusive<f64>) -> f64 {
    let (a1, a2) = (*from_range.start(), *from_range.end());
    let (b1, b2) = (*to_range.start(), *to_range.end());

    b1 + (value - a1) * (b2 - b1) / (a2 - a1)
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
            self.paint(&painter);
            // Make sure we allocate what we used (everything)
            ui.expand_to_include_rect(painter.clip_rect());

            let n = 100;
            let mut sin_values: Vec<_> = (0..=n)
                .map(|i| remap(i as f64, 0.0..=n as f64, -TAU..=TAU))
                .map(|i| [i, i.sin()])
                .collect();

            let line = egui_plot::Line::new(sin_values.split_off(n / 2)).fill(-1.5);
            let polygon = egui_plot::Polygon::new(egui_plot::PlotPoints::from_parametric_callback(
                |t| (4.0 * t.sin() + 2.0 * t.cos(), 4.0 * t.cos() + 2.0 * t.sin()),
                0.0..TAU,
                100,
            ));

            let points = egui_plot::Points::new(sin_values).stems(-1.5).radius(1.0);
            let plot = egui_plot::Plot::new("items_demo")
                .legend(egui_plot::Legend::default().position(egui_plot::Corner::RightBottom))
                .show_x(false)
                .show_y(false)
                .y_axis_width(2)
                .allow_zoom(false)
                .auto_bounds_x()
                .auto_bounds_y()
                .data_aspect(1.0);

            ui.style_mut().spacing.item_spacing.x = 0.;

            let mut special_points: Vec<[f64; 2]> = Vec::new();
            let mut regular_points: Vec<[f64; 2]> = Vec::new();

            for i in 0..100 {
                for j in 0..100 {
                    let point = [(i as f64 + 0.5f64), (j as f64 + 0.5f64)];
                    if (j == self.y) && (i == self.x) {
                        special_points.push(point);
                    } else {
                        regular_points.push(point);
                    }
                }
            }

            let special_markers = egui_plot::Points::new(special_points)
                .filled(true)
                .radius(self.marker_size)
                .highlight(true)
                .color(egui::Color32::from_rgba_unmultiplied(255, 0, 0, 255)) // Red color
                .shape(egui_plot::MarkerShape::Square);

            let regular_markers = egui_plot::Points::new(regular_points)
                .filled(true)
                .radius(self.marker_size)
                .highlight(true)
                .color(egui::Color32::from_rgba_unmultiplied(255, 255, 255, 25))
                .shape(egui_plot::MarkerShape::Square);

            plot.show(ui, |plot_ui| {
                plot_ui.hline(egui_plot::HLine::new(9.0).name("Lines horizontal"));
                plot_ui.hline(egui_plot::HLine::new(-9.0).name("Lines horizontal"));
                plot_ui.vline(egui_plot::VLine::new(9.0).name("Lines vertical"));
                plot_ui.vline(egui_plot::VLine::new(-9.0).name("Lines vertical"));
                plot_ui.line(line.name("Line with fill"));
                plot_ui.polygon(polygon.name("Convex polygon"));
                plot_ui.points(points.name("Points with stems"));
                plot_ui.text(
                    egui_plot::Text::new(egui_plot::PlotPoint::new(-3.0, -3.0), "wow").name("Text"),
                );
                plot_ui.text(
                    egui_plot::Text::new(egui_plot::PlotPoint::new(-2.0, 2.5), "so graph")
                        .name("Text"),
                );
                plot_ui.text(
                    egui_plot::Text::new(egui_plot::PlotPoint::new(3.0, 3.0), "much color")
                        .name("Text"),
                );
                plot_ui.text(
                    egui_plot::Text::new(egui_plot::PlotPoint::new(2.5, -2.0), "such plot")
                        .name("Text"),
                );
                let mut x: i32 = 0;
                let mut y: i32 = 0;
                if let Some(point) = plot_ui.pointer_coordinate() {
                    x = point.x as i32;
                    y = point.y as i32;
                }
                self.x = x;
                self.y = y;
                if (x.clamp(
                    (plot_ui.plot_bounds().min()[0] as i32),
                    (plot_ui.plot_bounds().max()[0] as i32),
                ) == self.x)
                    && (y.clamp(
                        (plot_ui.plot_bounds().min()[1] as i32),
                        (plot_ui.plot_bounds().max()[1] as i32),
                    ) == self.y)
                {
                    plot_ui.text(
                        egui_plot::Text::new(
                            egui_plot::PlotPoint::new(x, y),
                            egui::RichText::new(format!("cursor {x} {y}")).size(20.),
                        )
                        .name("Text")
                        .highlight(true),
                    );
                }
                plot_ui.points(special_markers);
                plot_ui.points(regular_markers);

                let dist = plot_ui
                    .screen_from_plot(egui_plot::PlotPoint::new(0.0, 0.0))
                    .distance(plot_ui.screen_from_plot(egui_plot::PlotPoint::new(100.0, 0.0)));

                // 1.8 for overlap
                self.marker_size = dist / (100. * 2.2);
            });
        });
    }
    #[allow(unused)]
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {}
}

impl DemoPanel {
    fn paint(&mut self, painter: &egui::Painter) {
        let rect = painter.clip_rect();
        //painter.rect_filled(
        //    rect,
        //    0.,
        //    egui::Color32::from_rgba_unmultiplied(255, 0, 255, 155),
        //)
        //let to_screen = emath::RectTransform::from_to(
        //    Rect::from_center_size(Pos2::ZERO, rect.square_proportions() / self.zoom),
        //    rect,
        //);
    }
}

use super::MAX_WRAP;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,

    #[serde(skip)]
    top_panel_font_size: f32,
    #[serde(skip)]
    top_panel_sep_width: f32,
    #[serde(skip)]
    top_panel_rects: Vec<egui::Rect>,
    #[serde(skip)]
    top_panel_height: f32,

    demo_settings_open: bool,
    demo_settings_label: String,

    demo_open: bool,
    demo_label: String,

    about_open: bool,
    about_label: String,

    app_settings_open: bool,
    app_settings_label: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            value: 2.7,
            top_panel_font_size: 20.0,
            top_panel_sep_width: 0.0,
            top_panel_rects: Vec::default(),
            top_panel_height: 40.,

            demo_settings_open: false,
            demo_settings_label: "⌨ Settings".to_owned(),

            demo_open: false,
            demo_label: "🧮 Demo".to_owned(),

            about_open: false,
            about_label: "📖 About".to_owned(),

            app_settings_open: false,
            app_settings_label: "⚙".to_owned(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        ctx.request_repaint();
        self.top_panel(ctx, _frame);

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("eframe template");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(&mut self.label);
            });

            ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                self.value += 1.0;
            }

            ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

impl TemplateApp {
    fn top_panel(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel")
            .exact_height(self.calc_top_panel_button_height())
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    // calc button dimensions:
                    self.calc_top_panel_button_rects(ui);
                    let bw = self.calc_top_panel_button_widths(ui);
                    let bh = self.calc_top_panel_button_height();
                    let offset = (ctx.screen_rect().width() - bw) / 2.;

                    ui.add_space(offset);
                    let mut button_text = "☀";
                    if ui.visuals().dark_mode {
                        button_text = "🌙";
                    }

                    let mut style = (*ctx.style()).clone();
                    style.text_styles = [(
                        egui::TextStyle::Button,
                        egui::FontId::new(18.0, egui::FontFamily::Proportional),
                    )]
                    .into();
                    ui.style_mut().text_styles = style.text_styles;

                    if ui
                        .add_sized(
                            [self.top_panel_rects[0].width(), bh],
                            egui::Button::new(button_text),
                        )
                        .clicked()
                    {
                        let visuals = if ui.visuals().dark_mode {
                            egui::Visuals::light()
                        } else {
                            egui::Visuals::dark()
                        };
                        ctx.set_visuals(visuals);
                    }
                    ui.add(egui::Separator::default().spacing(self.top_panel_sep_width));
                    if ui
                        .add_sized(
                            [self.top_panel_rects[1].width(), bh],
                            egui::SelectableLabel::new(
                                self.demo_settings_open,
                                self.demo_settings_label.clone(),
                            ),
                        )
                        .clicked()
                    {
                        self.demo_open = !self.demo_open;
                    }
                    if ui
                        .add_sized(
                            [self.top_panel_rects[2].width(), bh],
                            egui::SelectableLabel::new(self.demo_open, self.demo_label.clone()),
                        )
                        .clicked()
                    {
                        self.demo_open = true;
                    }
                    if ui
                        .add_sized(
                            [self.top_panel_rects[3].width(), bh],
                            egui::SelectableLabel::new(self.about_open, self.about_label.clone()),
                        )
                        .clicked()
                    {
                        self.about_open = !self.about_open;
                    }

                    ui.add(egui::Separator::default().spacing(self.top_panel_sep_width));

                    ui.scope(|ui| {
                        if ui
                            .add_sized(
                                [self.top_panel_rects[4].width(), bh],
                                egui::SelectableLabel::new(
                                    self.app_settings_open,
                                    self.app_settings_label.clone(),
                                ),
                            )
                            .clicked()
                        {
                            self.app_settings_open = !self.app_settings_open;
                        }
                    });
                });
            });
    }

    fn calc_top_panel_button_rects(&mut self, ui: &egui::Ui) {
        self.top_panel_rects.clear();

        self.top_panel_rects.push(
            ui.painter()
                .layout(
                    "☀".to_owned(),
                    egui::FontId::new(self.top_panel_font_size, egui::FontFamily::Proportional),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );

        self.top_panel_rects.push(
            ui.painter()
                .layout(
                    self.demo_settings_label.clone(),
                    egui::FontId::new(self.top_panel_font_size, egui::FontFamily::Proportional),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );

        self.top_panel_rects.push(
            ui.painter()
                .layout(
                    self.demo_label.clone(),
                    egui::FontId::new(self.top_panel_font_size, egui::FontFamily::Proportional),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );

        self.top_panel_rects.push(
            ui.painter()
                .layout(
                    self.about_label.clone(),
                    egui::FontId::new(self.top_panel_font_size, egui::FontFamily::Proportional),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );

        self.top_panel_rects.push(
            ui.painter()
                .layout(
                    self.app_settings_label.clone(),
                    egui::FontId::new(self.top_panel_font_size, egui::FontFamily::Proportional),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );
    }

    fn calc_top_panel_button_widths(&mut self, ui: &egui::Ui) -> f32 {
        let mut width = 0.;
        let item_spacing_x = ui.style().spacing.item_spacing.x;
        let button_padding_x = ui.style().spacing.button_padding.x;

        for r in self.top_panel_rects.iter() {
            width += r.width();
            width += button_padding_x * 2.;
            width += item_spacing_x;
        }

        width += item_spacing_x;

        width
    }

    fn calc_top_panel_button_height(&mut self) -> f32 {
        // Look at the tallest button
        self.top_panel_rects
            .iter()
            .map(|r| r.height())
            .fold(f32::NEG_INFINITY, |a, b| a.max(b))
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

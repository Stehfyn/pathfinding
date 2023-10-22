use super::Panel;
use super::MAX_WRAP;
use super::fixed_demo_label;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TopPanel {
    pub open: bool,
    #[serde(skip)]
    font_size: f32,
    #[serde(skip)]
    font_scale: f32,
    #[serde(skip)]
    sep_width: f32,
    #[serde(skip)]
    height: f32,
    #[serde(skip)]
    widget_rects: Vec<egui::Rect>,
    #[serde(skip)]
    label: String,

    #[serde(skip)]
    demo_settings_open: bool,
    #[serde(skip)]
    demo_settings_label: String,

    #[serde(skip)]
    demo_open: bool,
    #[serde(skip)]
    demo_label: String,

    #[serde(skip)]
    about_open: bool,
    #[serde(skip)]
    about_label: String,

    #[serde(skip)]
    app_settings_open: bool,
    #[serde(skip)]
    app_settings_label: String,
}

impl Default for TopPanel {
    fn default() -> Self {
        let fixed_demo_label = fixed_demo_label();
        Self {
            open: true,
            font_size: 20.0,
            font_scale: 1.0,
            sep_width: 0.0,
            height: 40.0,
            widget_rects: Vec::default(),
            label: "".to_owned(),

            demo_settings_open: false,
            demo_settings_label: "🖧 Configure".to_owned(),

            demo_open: false,
            demo_label: fixed_demo_label,

            about_open: false,
            about_label: "📖 About".to_owned(),

            app_settings_open: false,
            app_settings_label: "⚙".to_owned(),
        }
    }
}

impl Panel for TopPanel {
    #[allow(unused_variables)]
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel")
            .exact_height(self.calc_button_height())
            .show(ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    self.calc_button_rects(ui);
                    let bw = self.calc_button_widths(ui);
                    let bh = self.calc_button_height();
                    let offset = (ctx.screen_rect().width() - bw) / 2.;

                    ui.add_space(offset);
                    let mut button_text = "☀";
                    if ui.visuals().dark_mode {
                        button_text = "🌙";
                    }

                    let mut style = (*ctx.style()).clone();
                    if let Some(text_style) = style.text_styles.get_mut(&egui::TextStyle::Button) {
                        *text_style = egui::FontId::new(
                            self.font_size * self.font_scale,
                            egui::FontFamily::Proportional,
                        );
                    }
                    ui.style_mut().text_styles = style.text_styles;

                    if ui
                        .add_sized(
                            [self.widget_rects[0].width(), bh],
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
                    ui.add(egui::Separator::default().spacing(self.sep_width));
                    if ui
                        .add_sized(
                            [self.widget_rects[1].width(), bh],
                            egui::SelectableLabel::new(
                                self.demo_settings_open,
                                self.demo_settings_label.clone(),
                            ),
                        )
                        .clicked()
                    {
                        self.demo_settings_open = !self.demo_settings_open;
                    }

                    if ui
                        .add_sized(
                            [self.widget_rects[2].width(), bh],
                            egui::SelectableLabel::new(self.demo_open, self.demo_label.clone()),
                        )
                        .clicked()
                    {
                        self.demo_open = true;
                    }
                    if ui
                        .add_sized(
                            [self.widget_rects[3].width(), bh],
                            egui::SelectableLabel::new(self.about_open, self.about_label.clone()),
                        )
                        .clicked()
                    {
                        self.about_open = !self.about_open;
                    }

                    ui.add(egui::Separator::default().spacing(self.sep_width));

                    ui.scope(|ui| {
                        if ui
                            .add_sized(
                                [self.widget_rects[4].width(), bh],
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
    #[allow(unused)]
    fn ui(&mut self, ui: &mut egui::Ui, frame: &mut eframe::Frame) {}
}

impl TopPanel {
    pub fn is_demo_settings_open(&self) -> bool {
        self.demo_settings_open
    }

    pub fn is_app_settings_open(&self) -> bool {
        self.app_settings_open
    }

    pub fn set_font_scale(&mut self, scale: f32) {
        self.font_scale = scale
    }
}
impl TopPanel {
    fn calc_button_rects(&mut self, ui: &egui::Ui) {
        self.widget_rects.clear();

        self.widget_rects.push(
            ui.painter()
                .layout(
                    "☀".to_owned(),
                    egui::FontId::new(
                        self.font_size * self.font_scale,
                        egui::FontFamily::Proportional,
                    ),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );

        self.widget_rects.push(
            ui.painter()
                .layout(
                    self.demo_settings_label.clone(),
                    egui::FontId::new(
                        self.font_size * self.font_scale,
                        egui::FontFamily::Proportional,
                    ),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );

        self.widget_rects.push(
            ui.painter()
                .layout(
                    self.demo_label.clone(),
                    egui::FontId::new(
                        self.font_size * self.font_scale,
                        egui::FontFamily::Proportional,
                    ),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );

        self.widget_rects.push(
            ui.painter()
                .layout(
                    self.about_label.clone(),
                    egui::FontId::new(
                        self.font_size * self.font_scale,
                        egui::FontFamily::Proportional,
                    ),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );

        self.widget_rects.push(
            ui.painter()
                .layout(
                    self.app_settings_label.clone(),
                    egui::FontId::new(
                        self.font_size * self.font_scale,
                        egui::FontFamily::Proportional,
                    ),
                    egui::Color32::default(),
                    MAX_WRAP,
                )
                .rect,
        );
    }

    fn calc_button_widths(&mut self, ui: &egui::Ui) -> f32 {
        let mut width = 0.;
        let item_spacing_x = ui.style().spacing.item_spacing.x;
        let button_padding_x = ui.style().spacing.button_padding.x;

        for r in self.widget_rects.iter() {
            width += r.width();
            width += button_padding_x * 2.;
            width += item_spacing_x;
        }

        width += item_spacing_x;
        width += item_spacing_x;

        width
    }

    fn calc_button_height(&mut self) -> f32 {
        // Look at the tallest button
        self.widget_rects
            .iter()
            .map(|r| r.height())
            .fold(f32::NEG_INFINITY, |a, b| a.max(b))
    }
}

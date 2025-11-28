use eframe::egui;

#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

impl Theme {
    pub fn apply(&self, ctx: &egui::Context) {
        match self {
            Theme::Light => ctx.set_visuals(egui::Visuals::light()),
            Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
        }
    }

    pub fn next(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

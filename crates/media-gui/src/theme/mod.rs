pub mod style;

pub fn setup(ctx: &egui::Context) {
    let mut visuals = egui::Visuals::dark();
    visuals.window_corner_radius = egui::CornerRadius::same(6);
    visuals.panel_fill = egui::Color32::from_rgb(25, 25, 30);
    visuals.window_fill = egui::Color32::from_rgb(30, 30, 35);
    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(8.0, 6.0);
    ctx.set_style(style);
}

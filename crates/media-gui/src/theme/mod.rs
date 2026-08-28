pub mod style;

pub fn setup(ctx: &egui::Context) {
    setup_fonts(ctx);

    let mut visuals = egui::Visuals::dark();
    visuals.window_corner_radius = egui::CornerRadius::same(6);
    visuals.panel_fill = egui::Color32::from_rgb(25, 25, 30);
    visuals.window_fill = egui::Color32::from_rgb(30, 30, 35);
    ctx.set_visuals(visuals);

    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = egui::vec2(10.0, 8.0);
    style.spacing.window_margin = egui::Margin::same(12);
    style.spacing.button_padding = egui::vec2(14.0, 6.0);
    style.spacing.indent = 24.0;

    style.text_styles.insert(
        egui::TextStyle::Heading,
        egui::FontId::new(22.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Body,
        egui::FontId::new(16.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Button,
        egui::FontId::new(15.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Small,
        egui::FontId::new(13.0, egui::FontFamily::Proportional),
    );
    style.text_styles.insert(
        egui::TextStyle::Monospace,
        egui::FontId::new(15.0, egui::FontFamily::Monospace),
    );

    ctx.set_style(style);
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    let cjk_font = load_cjk_font();
    if let Some((name, data)) = cjk_font {
        fonts
            .font_data
            .insert(name.clone(), std::sync::Arc::new(data));

        for family in [egui::FontFamily::Proportional, egui::FontFamily::Monospace] {
            fonts.families.entry(family).or_default().push(name.clone());
        }
    }

    ctx.set_fonts(fonts);
}

fn load_cjk_font() -> Option<(String, egui::FontData)> {
    let candidates: &[(&str, &str)] = &[
        ("Microsoft YaHei", r"C:\Windows\Fonts\msyh.ttc"),
        ("Microsoft YaHei", r"C:\Windows\Fonts\msyh.ttf"),
        ("SimHei", r"C:\Windows\Fonts\simhei.ttf"),
        ("SimSun", r"C:\Windows\Fonts\simsun.ttc"),
        (
            "Noto Sans CJK SC",
            "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        ),
        (
            "Noto Sans CJK SC",
            "/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc",
        ),
    ];

    for (name, path) in candidates {
        if let Ok(data) = std::fs::read(path) {
            tracing::info!("Loaded CJK font: {} from {}", name, path);
            return Some((name.to_string(), egui::FontData::from_owned(data)));
        }
    }

    tracing::warn!("No CJK font found, Chinese characters may not display");
    None
}

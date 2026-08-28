mod app;
mod i18n;
mod pages;
mod state;
mod theme;
mod widgets;

fn main() -> eframe::Result {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1100.0, 700.0])
            .with_min_inner_size([800.0, 500.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Media RS",
        options,
        Box::new(|cc| {
            theme::setup(&cc.egui_ctx);
            Ok(Box::new(app::App::new(cc)))
        }),
    )
}

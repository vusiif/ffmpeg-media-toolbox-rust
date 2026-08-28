use crate::state::AppState;

pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading("Settings");
    ui.add_space(8.0);

    ui.group(|ui| {
        ui.label("FFmpeg Status:");
        if state.ffmpeg_valid {
            ui.colored_label(egui::Color32::GREEN, "Available");
        } else {
            ui.colored_label(egui::Color32::RED, "Not Found");
        }
    });

    ui.add_space(16.0);

    ui.group(|ui| {
        ui.label("About");
        ui.label("Media RS v0.1.0");
        ui.label("Fast media processor powered by FFmpeg");
        ui.label("Rust + egui");
    });
}

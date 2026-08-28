use crate::state::AppState;

pub fn show(ui: &mut egui::Ui, state: &mut AppState) {
    ui.heading("Presets");
    ui.add_space(8.0);

    egui::ScrollArea::vertical().show(ui, |ui| {
        for preset in &state.presets {
            let type_label = match preset.preset_type {
                media_core::preset::preset::PresetType::Video => "Video",
                media_core::preset::preset::PresetType::Audio => "Audio",
                media_core::preset::preset::PresetType::Image => "Image",
            };

            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.strong(&preset.name);
                    ui.label(format!("[{}]", type_label));
                });
                ui.label(&preset.description);
            });
            ui.add_space(4.0);
        }
    });
}

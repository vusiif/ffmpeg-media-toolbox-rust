use std::path::PathBuf;

use crate::app::GuiCommand;
use crate::state::AppState;

pub fn show(
    ui: &mut egui::Ui,
    state: &mut AppState,
    tx: tokio::sync::mpsc::UnboundedSender<GuiCommand>,
) {
    ui.heading("Batch Processing");
    ui.add_space(8.0);

    if !state.ffmpeg_valid {
        ui.colored_label(egui::Color32::RED, "FFmpeg not found!");
        return;
    }

    ui.horizontal(|ui| {
        ui.label("Preset:");
        egui::ComboBox::from_id_salt("batch_preset_picker")
            .selected_text(
                state
                    .selected_preset
                    .as_ref()
                    .map(|p| p.name.as_str())
                    .unwrap_or("None"),
            )
            .show_ui(ui, |ui| {
                for preset in &state.presets {
                    if ui
                        .selectable_label(
                            state
                                .selected_preset
                                .as_ref()
                                .map(|p| p.name == preset.name)
                                .unwrap_or(false),
                            &preset.name,
                        )
                        .clicked()
                    {
                        state.selected_preset = Some(preset.clone());
                    }
                }
            });
    });

    ui.add_space(8.0);

    if ui.button("Add Directory...").clicked() {
        if let Some(dir) = rfd::FileDialog::new().pick_folder() {
            let _ = tx.send(GuiCommand::AddDirectory(dir));
        }
    }

    ui.add_space(4.0);

    if ui.button("Add Files...").clicked() {
        if let Some(files) = rfd::FileDialog::new().pick_files() {
            let _ = tx.send(GuiCommand::AddFiles(files));
        }
    }

    ui.add_space(8.0);

    if !state.files.is_empty() {
        ui.label(format!("{} file(s) queued", state.files.len()));

        egui::ScrollArea::vertical()
            .max_height(300.0)
            .show(ui, |ui| {
                let files: Vec<(usize, String)> = state
                    .files
                    .iter()
                    .enumerate()
                    .map(|(i, p)| {
                        (
                            i,
                            p.file_name()
                                .map(|n| n.to_string_lossy().to_string())
                                .unwrap_or_else(|| p.display().to_string()),
                        )
                    })
                    .collect();

                for (i, name) in &files {
                    ui.horizontal(|ui| {
                        ui.label(name);
                        if ui.small_button("x").clicked() {
                            state.remove_file(*i);
                        }
                    });
                }
            });

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            if ui.button("Start Batch").clicked() {
                let files: Vec<PathBuf> = state.files.drain(..).collect();
                for file in files {
                    let _ = tx.send(GuiCommand::StartJob(file));
                }
            }
            if ui.button("Clear All").clicked() {
                state.clear_files();
            }
        });
    }
}

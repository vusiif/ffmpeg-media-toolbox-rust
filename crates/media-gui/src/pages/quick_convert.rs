use std::path::PathBuf;

use crate::app::GuiCommand;
use crate::i18n::Key;
use crate::state::AppState;
use crate::widgets;

pub fn show(
    ui: &mut egui::Ui,
    state: &mut AppState,
    tx: tokio::sync::mpsc::UnboundedSender<GuiCommand>,
) {
    ui.heading(state.lang.t(Key::QuickConvert));
    ui.add_space(8.0);

    if !state.ffmpeg_valid {
        ui.colored_label(
            egui::Color32::RED,
            format!(
                "{} {}",
                state.lang.t(Key::FFmpegNotFound),
                state.lang.t(Key::FfmpegNotFoundMsg)
            ),
        );
        return;
    }

    widgets::file_drop::show(ui, &tx, &state.lang);

    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label(state.lang.t(Key::Preset));
        egui::ComboBox::from_id_salt("preset_picker")
            .selected_text(
                state
                    .selected_preset
                    .as_ref()
                    .map(|p| p.name.as_str())
                    .unwrap_or(state.lang.t(Key::None)),
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

    if ui.button(state.lang.t(Key::AddFiles)).clicked() {
        if let Some(files) = rfd::FileDialog::new().pick_files() {
            state.add_files(files);
        }
    }

    ui.add_space(8.0);

    if !state.files.is_empty() {
        ui.label(
            state
                .lang
                .t(Key::FilesSelected)
                .replace("{}", &state.files.len().to_string()),
        );

        ui.add_space(4.0);

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

        ui.add_space(8.0);

        ui.horizontal(|ui| {
            if ui.button(state.lang.t(Key::StartConvert)).clicked() {
                let files: Vec<PathBuf> = state.files.drain(..).collect();
                for file in files {
                    let _ = tx.send(GuiCommand::StartJob(file));
                }
            }
            if ui.button(state.lang.t(Key::Clear)).clicked() {
                state.clear_files();
            }
        });
    }
}

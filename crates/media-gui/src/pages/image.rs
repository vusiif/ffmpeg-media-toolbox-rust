use std::path::PathBuf;

use crate::app::GuiCommand;
use crate::i18n::Key;
use crate::state::AppState;

pub fn show(
    ui: &mut egui::Ui,
    state: &mut AppState,
    tx: tokio::sync::mpsc::UnboundedSender<GuiCommand>,
) {
    ui.heading(state.lang.t(Key::ImageTools));
    ui.add_space(8.0);

    ui.label(state.lang.t(Key::ImageOps));
    ui.add_space(8.0);

    if ui.button(state.lang.t(Key::AddFiles)).clicked() {
        if let Some(files) = rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg", "webp", "bmp", "tiff"])
            .pick_files()
        {
            state.add_files(files);
        }
    }

    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label(state.lang.t(Key::OutputFormat));
        egui::ComboBox::from_id_salt("image_format")
            .selected_text("webp")
            .show_ui(ui, |ui| {
                for fmt in &["webp", "png", "jpg", "bmp"] {
                    let _ = ui.selectable_label(false, *fmt);
                }
            });
    });

    ui.add_space(8.0);

    if !state.files.is_empty() {
        ui.label(
            state
                .lang
                .t(Key::ImagesSelected)
                .replace("{}", &state.files.len().to_string()),
        );

        egui::ScrollArea::vertical()
            .max_height(200.0)
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
            if ui.button(state.lang.t(Key::ConvertAll)).clicked() {
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

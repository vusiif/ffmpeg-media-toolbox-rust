use std::path::PathBuf;

use crate::app::GuiCommand;
use crate::state::AppState;

pub fn show(
    ui: &mut egui::Ui,
    state: &mut AppState,
    tx: tokio::sync::mpsc::UnboundedSender<GuiCommand>,
) {
    ui.heading("Image Tools");
    ui.add_space(8.0);

    ui.label("Image operations: Convert, Resize, Crop, Rotate, Flip");
    ui.add_space(8.0);

    if ui.button("Add Images...").clicked() {
        if let Some(files) = rfd::FileDialog::new()
            .add_filter("Images", &["png", "jpg", "jpeg", "webp", "bmp", "tiff"])
            .pick_files()
        {
            state.add_files(files);
        }
    }

    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label("Output format:");
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
        ui.label(format!("{} image(s) selected", state.files.len()));

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
            if ui.button("Convert All").clicked() {
                let files: Vec<PathBuf> = state.files.drain(..).collect();
                for file in files {
                    let _ = tx.send(GuiCommand::StartJob(file));
                }
            }
            if ui.button("Clear").clicked() {
                state.clear_files();
            }
        });
    }
}

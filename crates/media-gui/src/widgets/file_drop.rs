use std::path::PathBuf;

use crate::app::GuiCommand;
use crate::i18n::{Key, Lang};

pub fn show(ui: &mut egui::Ui, tx: &tokio::sync::mpsc::UnboundedSender<GuiCommand>, lang: &Lang) {
    let response =
        ui.allocate_response(egui::vec2(ui.available_width(), 80.0), egui::Sense::hover());

    let rect = response.rect;
    let painter = ui.painter_at(rect);

    let dropped_files = ui.input(|i| i.raw.dropped_files.clone());

    if !dropped_files.is_empty() {
        let paths: Vec<PathBuf> = dropped_files.into_iter().filter_map(|f| f.path).collect();

        if !paths.is_empty() {
            let _ = tx.send(GuiCommand::AddFiles(paths));
        }
    }

    let bg_color = if response.hovered() {
        ui.visuals().selection.bg_fill
    } else {
        ui.visuals().extreme_bg_color
    };

    painter.rect_filled(rect, egui::CornerRadius::same(4), bg_color);
    painter.rect_stroke(
        rect,
        egui::CornerRadius::same(4),
        egui::Stroke::new(1.0, ui.visuals().weak_text_color()),
        egui::StrokeKind::Outside,
    );

    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        lang.t(Key::DropFilesHere),
        egui::FontId::proportional(16.0),
        ui.visuals().text_color(),
    );
}

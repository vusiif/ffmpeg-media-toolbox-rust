use crate::app::GuiCommand;
use crate::i18n::{Key, Language};
use crate::state::AppState;

pub fn show(
    ui: &mut egui::Ui,
    state: &mut AppState,
    tx: tokio::sync::mpsc::UnboundedSender<GuiCommand>,
) {
    ui.heading(state.lang.t(Key::Settings));
    ui.add_space(16.0);

    ui.group(|ui| {
        ui.label(state.lang.t(Key::LanguageSwitch));
        ui.horizontal(|ui| {
            for lang in Language::all() {
                let selected = state.lang.lang == *lang;
                if ui.selectable_label(selected, lang.label()).clicked() {
                    let _ = tx.send(GuiCommand::SetLanguage(*lang));
                }
            }
        });
    });

    ui.add_space(16.0);

    ui.group(|ui| {
        ui.label(state.lang.t(Key::FFmpegStatus));
        if state.ffmpeg_valid {
            ui.colored_label(egui::Color32::GREEN, state.lang.t(Key::Available));
        } else {
            ui.colored_label(egui::Color32::RED, state.lang.t(Key::NotFound));
        }
    });

    ui.add_space(16.0);

    ui.group(|ui| {
        ui.label(state.lang.t(Key::About));
        ui.label("Media RS v0.1.0");
        ui.label(state.lang.t(Key::Description));
        ui.label("Rust + egui");
    });
}

use crate::app::GuiCommand;
use crate::i18n::{Key, Lang};

pub fn show(
    ui: &mut egui::Ui,
    tx: &tokio::sync::mpsc::UnboundedSender<GuiCommand>,
    id: &str,
    name: &str,
    status: &str,
    progress: Option<f64>,
    lang: &Lang,
) {
    ui.horizontal(|ui| {
        let status_color = match status {
            s if s == lang.t(Key::Running) => egui::Color32::from_rgb(50, 150, 255),
            s if s == lang.t(Key::Done) => egui::Color32::from_rgb(50, 200, 50),
            s if s == lang.t(Key::Failed) => egui::Color32::from_rgb(255, 80, 80),
            s if s == lang.t(Key::Cancelled) => egui::Color32::GRAY,
            _ => egui::Color32::from_rgb(200, 200, 100),
        };

        ui.colored_label(status_color, format!("{:<12}", status));
        ui.label(format!("{:<30}", name));

        if let Some(pct) = progress {
            let bar = egui::ProgressBar::new(pct as f32 / 100.0)
                .show_percentage()
                .desired_width(100.0);
            ui.add(bar);
        }

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let is_failed = status == lang.t(Key::Failed) || status == lang.t(Key::Cancelled);
            let is_running = status == lang.t(Key::Running)
                || status == lang.t(Key::Pending)
                || status == lang.t(Key::Preparing)
                || status == lang.t(Key::Waiting);

            if is_failed && ui.small_button(lang.t(Key::Retry)).clicked() {
                let _ = tx.send(GuiCommand::RetryJob(id.to_string()));
            }
            if !is_running && ui.small_button(lang.t(Key::Remove)).clicked() {
                let _ = tx.send(GuiCommand::RemoveJob(id.to_string()));
            }
            if is_running && ui.small_button(lang.t(Key::Cancel)).clicked() {
                let _ = tx.send(GuiCommand::CancelJob(id.to_string()));
            }
        });
    });

    ui.add_space(2.0);
}

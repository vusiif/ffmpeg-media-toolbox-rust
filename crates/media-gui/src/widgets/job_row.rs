use crate::app::GuiCommand;

pub fn show(
    ui: &mut egui::Ui,
    tx: &tokio::sync::mpsc::UnboundedSender<GuiCommand>,
    id: &str,
    name: &str,
    status: &str,
    progress: Option<f64>,
) {
    ui.horizontal(|ui| {
        let status_color = match status {
            "RUNNING" => egui::Color32::from_rgb(50, 150, 255),
            "DONE" => egui::Color32::from_rgb(50, 200, 50),
            "FAILED" => egui::Color32::from_rgb(255, 80, 80),
            "CANCELLED" => egui::Color32::GRAY,
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
            if (status == "FAILED" || status == "CANCELLED") && ui.small_button("Retry").clicked() {
                let _ = tx.send(GuiCommand::RetryJob(id.to_string()));
            }
            if status != "RUNNING" && ui.small_button("Remove").clicked() {
                let _ = tx.send(GuiCommand::RemoveJob(id.to_string()));
            }
            if (status == "RUNNING" || status == "WAITING" || status == "PREPARING")
                && ui.small_button("Cancel").clicked()
            {
                let _ = tx.send(GuiCommand::CancelJob(id.to_string()));
            }
        });
    });

    ui.add_space(2.0);
}

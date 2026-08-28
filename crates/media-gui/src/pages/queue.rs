use crate::app::GuiCommand;
use crate::state::AppState;
use crate::widgets;

pub fn show(ui: &mut egui::Ui, state: &mut AppState, tx: tokio::sync::mpsc::UnboundedSender<GuiCommand>) {
    ui.heading("Job Queue");
    ui.add_space(8.0);

    let stats = state.queue_stats();

    ui.horizontal(|ui| {
        ui.label(format!("Total: {}", stats.total));
        ui.label(format!("Running: {}", stats.running));
        ui.label(format!("Pending: {}", stats.pending));
        ui.label(format!("Completed: {}", stats.completed));
        ui.label(format!("Failed: {}", stats.failed));

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button("Clear Completed").clicked() {
                let _ = tx.send(GuiCommand::ClearCompleted);
            }
        });
    });

    ui.add_space(8.0);

    egui::ScrollArea::vertical().show(ui, |ui| {
        let jobs: Vec<(String, String, String, Option<f64>)> = state
            .queue
            .all_jobs()
            .iter()
            .map(|job| {
                let status = match &job.status {
                    media_core::jobs::job::JobStatus::Pending => "WAITING".to_string(),
                    media_core::jobs::job::JobStatus::Preparing => "PREPARING".to_string(),
                    media_core::jobs::job::JobStatus::Running => "RUNNING".to_string(),
                    media_core::jobs::job::JobStatus::Completed => "DONE".to_string(),
                    media_core::jobs::job::JobStatus::Failed(_) => "FAILED".to_string(),
                    media_core::jobs::job::JobStatus::Cancelled => "CANCELLED".to_string(),
                };
                let progress = job.progress.as_ref().and_then(|p| p.percentage);
                (job.id.0.clone(), job.name(), status, progress)
            })
            .collect();

        for (id, name, status, progress) in &jobs {
            widgets::job_row::show(ui, &tx, id, name, status, *progress);
        }

        if jobs.is_empty() {
            ui.label("No jobs in queue");
        }
    });
}

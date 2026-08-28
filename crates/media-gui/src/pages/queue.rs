use crate::app::GuiCommand;
use crate::i18n::Key;
use crate::state::AppState;
use crate::widgets;

pub fn show(
    ui: &mut egui::Ui,
    state: &mut AppState,
    tx: tokio::sync::mpsc::UnboundedSender<GuiCommand>,
) {
    ui.heading(state.lang.t(Key::JobQueue));
    ui.add_space(8.0);

    let stats = state.queue_stats();

    ui.horizontal(|ui| {
        ui.label(format!("{} {}", state.lang.t(Key::Total), stats.total));
        ui.label(format!("{} {}", state.lang.t(Key::Running), stats.running));
        ui.label(format!("{} {}", state.lang.t(Key::Pending), stats.pending));
        ui.label(format!("{} {}", state.lang.t(Key::Done), stats.completed));
        ui.label(format!("{} {}", state.lang.t(Key::Failed), stats.failed));

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(state.lang.t(Key::ClearCompleted)).clicked() {
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
                    media_core::jobs::job::JobStatus::Pending => {
                        state.lang.t(Key::Pending).to_string()
                    }
                    media_core::jobs::job::JobStatus::Preparing => {
                        state.lang.t(Key::Preparing).to_string()
                    }
                    media_core::jobs::job::JobStatus::Running => {
                        state.lang.t(Key::Running).to_string()
                    }
                    media_core::jobs::job::JobStatus::Completed => {
                        state.lang.t(Key::Done).to_string()
                    }
                    media_core::jobs::job::JobStatus::Failed(_) => {
                        state.lang.t(Key::Failed).to_string()
                    }
                    media_core::jobs::job::JobStatus::Cancelled => {
                        state.lang.t(Key::Cancelled).to_string()
                    }
                };
                let progress = job.progress.as_ref().and_then(|p| p.percentage);
                (job.id.0.clone(), job.name(), status, progress)
            })
            .collect();

        for (id, name, status, progress) in &jobs {
            widgets::job_row::show(ui, &tx, id, name, status, *progress, &state.lang);
        }

        if jobs.is_empty() {
            ui.label(state.lang.t(Key::NoJobs));
        }
    });
}

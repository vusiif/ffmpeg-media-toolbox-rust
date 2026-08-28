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

    let stats = state.job_display_stats();

    ui.horizontal(|ui| {
        ui.label(format!("{} {}", state.lang.t(Key::Running), stats.0));
        ui.label(format!("{} {}", state.lang.t(Key::Pending), stats.1));
        ui.label(format!("{} {}", state.lang.t(Key::Done), stats.2));
        ui.label(format!("{} {}", state.lang.t(Key::Failed), stats.3));

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui.button(state.lang.t(Key::ClearCompleted)).clicked() {
                let _ = tx.send(GuiCommand::ClearCompleted);
            }
        });
    });

    ui.add_space(8.0);

    egui::ScrollArea::vertical().show(ui, |ui| {
        let jobs: Vec<(String, String, String, Option<f64>)> = state
            .jobs
            .iter()
            .map(|job| {
                let status = job.status.label(&state.lang);
                let progress = job.progress.as_ref().and_then(|p| p.percentage);
                (job.id.clone(), job.name.clone(), status, progress)
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

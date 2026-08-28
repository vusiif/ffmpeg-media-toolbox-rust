use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::mpsc;

use media_core::ffmpeg::locator::FfmpegLocator;
use media_core::jobs::executor::JobEvent;
use media_core::jobs::job::JobRequest;

use crate::i18n::Language;
use crate::pages::Page;
use crate::state::AppState;

pub struct App {
    state: AppState,
    current_page: Page,
    cmd_tx: mpsc::UnboundedSender<GuiCommand>,
    cmd_rx: mpsc::UnboundedReceiver<GuiCommand>,
    event_rx: mpsc::UnboundedReceiver<JobEvent>,
    job_tx: Option<mpsc::UnboundedSender<JobRequest>>,
    cancel_tx: Option<tokio::sync::watch::Sender<Option<String>>>,
    _runtime: tokio::runtime::Runtime,
}

pub enum GuiCommand {
    AddFiles(Vec<PathBuf>),
    AddDirectory(PathBuf),
    StartJob(PathBuf),
    CancelJob(String),
    RemoveJob(String),
    RetryJob(String),
    ClearCompleted,
    SwitchPage(Page),
    SetLanguage(Language),
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
        let (event_tx, event_rx) = mpsc::unbounded_channel();
        let (job_tx, job_rx) = mpsc::unbounded_channel::<JobRequest>();

        let runtime = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");

        let ctx = cc.egui_ctx.clone();
        let locator = match FfmpegLocator::new() {
            Ok(loc) => Some(Arc::new(loc)),
            Err(e) => {
                tracing::warn!("FFmpeg not found: {}", e);
                None
            }
        };

        let mut cancel_tx = None;
        if let Some(locator) = locator {
            let executor = Arc::new(media_core::jobs::executor::JobExecutor::new(
                (*locator).clone(),
            ));
            cancel_tx = Some(executor.cancel_sender());
            runtime.spawn(run_job_loop(executor, job_rx, event_tx, ctx));
        }

        Self {
            state: AppState::new(),
            current_page: Page::QuickConvert,
            cmd_tx,
            cmd_rx,
            event_rx,
            job_tx: Some(job_tx),
            cancel_tx,
            _runtime: runtime,
        }
    }

    fn process_events(&mut self, ctx: &egui::Context) {
        let mut got_events = false;
        while let Ok(event) = self.event_rx.try_recv() {
            got_events = true;
            match event {
                JobEvent::Started(id) => {
                    self.state
                        .update_job_status(&id.0, media_core::jobs::job::JobStatus::Running);
                }
                JobEvent::Progress(id, info) => {
                    self.state.update_job_progress(&id.0, info);
                }
                JobEvent::Completed(id) => {
                    self.state
                        .update_job_status(&id.0, media_core::jobs::job::JobStatus::Completed);
                }
                JobEvent::Failed(id, msg) => {
                    self.state
                        .update_job_status(&id.0, media_core::jobs::job::JobStatus::Failed(msg));
                }
                JobEvent::Cancelled(id) => {
                    self.state
                        .update_job_status(&id.0, media_core::jobs::job::JobStatus::Cancelled);
                }
            }
        }
        if got_events {
            ctx.request_repaint();
        }
    }

    fn process_commands(&mut self) {
        while let Ok(cmd) = self.cmd_rx.try_recv() {
            match cmd {
                GuiCommand::SwitchPage(page) => {
                    self.current_page = page;
                }
                GuiCommand::AddFiles(paths) => {
                    self.state.add_files(paths);
                }
                GuiCommand::AddDirectory(path) => {
                    self.state.add_directory(path);
                }
                GuiCommand::StartJob(path) => {
                    self.state.enqueue_and_send(path, self.job_tx.as_ref());
                }
                GuiCommand::CancelJob(id) => {
                    self.state.cancel_job(&id);
                    if let Some(ref tx) = self.cancel_tx {
                        let _ = tx.send(Some("cancel".to_string()));
                    }
                }
                GuiCommand::RemoveJob(id) => {
                    self.state.remove_job(&id);
                }
                GuiCommand::RetryJob(id) => {
                    self.state.retry_job(&id);
                }
                GuiCommand::ClearCompleted => {
                    self.state.clear_completed_jobs();
                }
                GuiCommand::SetLanguage(lang) => {
                    self.state.set_language(lang);
                }
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.process_events(ctx);
        self.process_commands();

        let lang = &self.state.lang;

        egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading(lang.t(crate::i18n::Key::AppTitle));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("⚙").clicked() {
                        self.current_page = Page::Settings;
                    }
                });
            });
        });

        egui::SidePanel::left("nav_panel")
            .resizable(false)
            .exact_width(170.0)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.add_space(12.0);
                    for page in Page::all() {
                        let selected = self.current_page == *page;
                        let label = page.label(lang);
                        if ui.selectable_label(selected, label).clicked() {
                            self.current_page = *page;
                        }
                    }
                });
            });

        egui::TopBottomPanel::bottom("status_bar")
            .exact_height(28.0)
            .show(ctx, |ui| {
                let stats = self.state.job_display_stats();
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "{} {} | {} {} | {} {} | {} {}",
                        lang.t(crate::i18n::Key::Running),
                        stats.0,
                        lang.t(crate::i18n::Key::Waiting),
                        stats.1,
                        lang.t(crate::i18n::Key::Done),
                        stats.2,
                        lang.t(crate::i18n::Key::Failed),
                        stats.3
                    ));
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            let tx = self.cmd_tx.clone();
            match self.current_page {
                Page::QuickConvert => {
                    crate::pages::quick_convert::show(ui, &mut self.state, tx);
                }
                Page::Batch => {
                    crate::pages::batch::show(ui, &mut self.state, tx);
                }
                Page::Image => {
                    crate::pages::image::show(ui, &mut self.state, tx);
                }
                Page::Queue => {
                    crate::pages::queue::show(ui, &mut self.state, tx);
                }
                Page::Presets => {
                    crate::pages::presets::show(ui, &mut self.state);
                }
                Page::Settings => {
                    crate::pages::settings::show(ui, &mut self.state, tx);
                }
            }
        });
    }
}

async fn run_job_loop(
    executor: Arc<media_core::jobs::executor::JobExecutor>,
    mut job_rx: mpsc::UnboundedReceiver<JobRequest>,
    event_tx: mpsc::UnboundedSender<JobEvent>,
    ctx: egui::Context,
) {
    while let Some(request) = job_rx.recv().await {
        let mut job = media_core::jobs::job::Job::new(request);
        let id = job.id.clone();

        tracing::info!("Starting job {}", id);

        if let Err(e) = executor.execute(&mut job, &event_tx).await {
            tracing::error!("Job {} failed: {}", id, e);
            let _ = event_tx.send(JobEvent::Failed(id, e.to_string()));
        }

        ctx.request_repaint();
    }
}

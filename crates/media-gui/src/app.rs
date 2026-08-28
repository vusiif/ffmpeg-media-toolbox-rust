use std::path::PathBuf;

use tokio::sync::mpsc;

use crate::pages::Page;
use crate::state::AppState;

pub struct App {
    state: AppState,
    current_page: Page,
    cmd_tx: mpsc::UnboundedSender<GuiCommand>,
    cmd_rx: mpsc::UnboundedReceiver<GuiCommand>,
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
    SetLanguage(crate::i18n::Language),
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        Self {
            state: AppState::new(),
            current_page: Page::QuickConvert,
            cmd_tx,
            cmd_rx,
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
                    self.state.start_convert_job(path);
                }
                GuiCommand::CancelJob(id) => {
                    self.state.cancel_job(&id);
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
                let stats = self.state.queue_stats();
                ui.horizontal(|ui| {
                    ui.label(format!(
                        "{} {} | {} {} | {} {} | {} {}",
                        lang.t(crate::i18n::Key::Running),
                        stats.running,
                        lang.t(crate::i18n::Key::Waiting),
                        stats.pending + stats.running,
                        lang.t(crate::i18n::Key::Done),
                        stats.completed,
                        lang.t(crate::i18n::Key::Failed),
                        stats.failed
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

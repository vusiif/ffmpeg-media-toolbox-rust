pub mod batch;
pub mod image;
pub mod presets;
pub mod queue;
pub mod quick_convert;
pub mod settings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    QuickConvert,
    Batch,
    Image,
    Queue,
    Presets,
    Settings,
}

impl Page {
    pub fn all() -> &'static [Page] {
        &[
            Page::QuickConvert,
            Page::Batch,
            Page::Image,
            Page::Queue,
            Page::Presets,
            Page::Settings,
        ]
    }

    pub fn label(self) -> &'static str {
        match self {
            Page::QuickConvert => "Quick Convert",
            Page::Batch => "Batch",
            Page::Image => "Image",
            Page::Queue => "Queue",
            Page::Presets => "Presets",
            Page::Settings => "Settings",
        }
    }
}

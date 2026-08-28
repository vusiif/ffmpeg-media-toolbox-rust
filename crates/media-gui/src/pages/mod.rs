pub mod batch;
pub mod image;
pub mod presets;
pub mod queue;
pub mod quick_convert;
pub mod settings;

use crate::i18n::{Key, Lang};

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

    pub fn label(self, lang: &Lang) -> &'static str {
        match self {
            Page::QuickConvert => lang.t(Key::QuickConvert),
            Page::Batch => lang.t(Key::Batch),
            Page::Image => lang.t(Key::Image),
            Page::Queue => lang.t(Key::Queue),
            Page::Presets => lang.t(Key::Presets),
            Page::Settings => lang.t(Key::Settings),
        }
    }
}

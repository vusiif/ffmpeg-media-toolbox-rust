use std::collections::VecDeque;
use std::time::Duration;

use super::job::JobId;

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub job_id: JobId,
    pub name: String,
    pub success: bool,
    pub error: Option<String>,
    pub duration: Option<Duration>,
}

pub struct JobHistory {
    entries: VecDeque<HistoryEntry>,
    max_size: usize,
}

impl JobHistory {
    pub fn new(max_size: usize) -> Self {
        Self {
            entries: VecDeque::new(),
            max_size,
        }
    }

    pub fn push(&mut self, entry: HistoryEntry) {
        if self.entries.len() >= self.max_size {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    pub fn entries(&self) -> &VecDeque<HistoryEntry> {
        &self.entries
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }
}

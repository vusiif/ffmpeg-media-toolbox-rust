use std::collections::HashMap;

use super::job::Workload;

pub struct Scheduler {
    concurrency: ConcurrencyMode,
    running: HashMap<Workload, usize>,
}

#[derive(Debug, Clone)]
pub enum ConcurrencyMode {
    Automatic,
    Fixed(usize),
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            concurrency: ConcurrencyMode::Automatic,
            running: HashMap::new(),
        }
    }

    pub fn set_concurrency(&mut self, mode: ConcurrencyMode) {
        self.concurrency = mode;
    }

    pub fn can_start(&self, workload: Workload) -> bool {
        let current = self.running.get(&workload).copied().unwrap_or(0);
        let max = self.max_for(workload);
        current < max
    }

    pub fn acquire(&mut self, workload: Workload) -> bool {
        if !self.can_start(workload) {
            return false;
        }
        *self.running.entry(workload).or_insert(0) += 1;
        true
    }

    pub fn release(&mut self, workload: Workload) {
        if let Some(count) = self.running.get_mut(&workload) {
            if *count > 0 {
                *count -= 1;
            }
        }
    }

    pub fn running_count(&self, workload: Workload) -> usize {
        self.running.get(&workload).copied().unwrap_or(0)
    }

    pub fn total_running(&self) -> usize {
        self.running.values().sum()
    }

    fn max_for(&self, workload: Workload) -> usize {
        match &self.concurrency {
            ConcurrencyMode::Automatic => workload.default_concurrency(),
            ConcurrencyMode::Fixed(n) => *n,
        }
    }
}

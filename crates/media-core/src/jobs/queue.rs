use std::collections::VecDeque;

use super::job::{Job, JobId, JobRequest, JobStatus};

pub struct JobQueue {
    jobs: VecDeque<Job>,
}

impl Default for JobQueue {
    fn default() -> Self {
        Self::new()
    }
}

impl JobQueue {
    pub fn new() -> Self {
        Self {
            jobs: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, request: JobRequest) -> JobId {
        let job = Job::new(request);
        let id = job.id.clone();
        self.jobs.push_back(job);
        id
    }

    pub fn enqueue_job(&mut self, job: Job) -> JobId {
        let id = job.id.clone();
        self.jobs.push_back(job);
        id
    }

    pub fn get(&self, id: &JobId) -> Option<&Job> {
        self.jobs.iter().find(|j| j.id == *id)
    }

    pub fn get_mut(&mut self, id: &JobId) -> Option<&mut Job> {
        self.jobs.iter_mut().find(|j| j.id == *id)
    }

    pub fn pending_jobs(&self) -> Vec<&Job> {
        self.jobs
            .iter()
            .filter(|j| j.status == JobStatus::Pending)
            .collect()
    }

    pub fn running_jobs(&self) -> Vec<&Job> {
        self.jobs
            .iter()
            .filter(|j| j.status == JobStatus::Running)
            .collect()
    }

    pub fn cancel(&mut self, id: &JobId) -> bool {
        if let Some(job) = self.get_mut(id) {
            if !job.is_terminal() {
                job.status = JobStatus::Cancelled;
                return true;
            }
        }
        false
    }

    pub fn remove(&mut self, id: &JobId) -> bool {
        let len_before = self.jobs.len();
        self.jobs.retain(|j| j.id != *id);
        self.jobs.len() < len_before
    }

    pub fn clear_completed(&mut self) {
        self.jobs.retain(|j| !j.is_terminal());
    }

    pub fn retry_failed(&mut self) {
        for job in self.jobs.iter_mut() {
            if matches!(job.status, JobStatus::Failed(_)) {
                job.status = JobStatus::Pending;
                job.progress = None;
                job.error_message = None;
                job.start_time = None;
                job.end_time = None;
                job.stderr_tail.clear();
            }
        }
    }

    pub fn all_jobs(&self) -> &VecDeque<Job> {
        &self.jobs
    }

    pub fn len(&self) -> usize {
        self.jobs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.jobs.is_empty()
    }

    pub fn stats(&self) -> QueueStats {
        let mut stats = QueueStats::default();
        for job in &self.jobs {
            match &job.status {
                JobStatus::Pending => stats.pending += 1,
                JobStatus::Preparing => stats.pending += 1,
                JobStatus::Running => stats.running += 1,
                JobStatus::Completed => stats.completed += 1,
                JobStatus::Failed(_) => stats.failed += 1,
                JobStatus::Cancelled => stats.cancelled += 1,
            }
        }
        stats.total = self.jobs.len();
        stats
    }
}

#[derive(Debug, Clone, Default)]
pub struct QueueStats {
    pub total: usize,
    pub pending: usize,
    pub running: usize,
    pub completed: usize,
    pub failed: usize,
    pub cancelled: usize,
}

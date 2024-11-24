use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub enum TaskStatus {
    Incomplete,
    Complete
}

pub trait Task {
    fn description(&self) -> &str;
    fn status(&self) -> &TaskStatus;
    fn mark_complete(&mut self);
}


impl Task for TaskStruct {
    fn description(&self) -> &str {
        &self.description
    }
    fn status(&self) -> &TaskStatus {
        &self.status
    }
    fn mark_complete(&mut self) {
        self.done = true;
        self.status = TaskStatus::Complete;
    }
}


pub struct TaskStruct {
    pub done: bool,
    pub name: String,
    pub description: String,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>
}

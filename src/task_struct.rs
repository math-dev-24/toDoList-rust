use chrono::{DateTime, Utc};

pub struct Task {
    pub done: bool,
    pub name: String,
    pub created_at: DateTime<Utc>
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jam {
    pub id: Ulid,

    pub created_by: Ulid,

    pub title: String,
    pub summary: Option<String>,
    pub description: Option<String>,

    pub starts: DateTime<Utc>,
    pub ends: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Submission {
    pub id: Ulid,

    pub jam_id: Ulid,

    pub icon: Option<Ulid>,

    pub title: String,
    pub summary: Option<String>,
    pub description: Option<String>,

    pub source_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionAuthor {
    pub submission_id: Ulid,

    pub author: Ulid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionMedia {
    pub media_id: Ulid,

    pub submission_id: Ulid,

    pub uploaded_by: Ulid,
}

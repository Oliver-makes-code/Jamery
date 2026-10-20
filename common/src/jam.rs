use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Jam {
    pub id: Ulid,

    #[serde(with = "crate::u64_as_string")]
    pub created_by: u64,

    pub title: String,
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
    pub description: Option<String>,

    pub source_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionAuthor {
    pub submission_id: Ulid,

    #[serde(with = "crate::u64_as_string")]
    pub author: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmissionMedia {
    pub media_id: Ulid,

    pub submission_id: Ulid,

    #[serde(with = "crate::u64_as_string")]
    pub uploaded_by: u64,
}

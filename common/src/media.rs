use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Media {
    pub id: Ulid,
    pub file_name: String,
    pub content_type: String,
}

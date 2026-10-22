use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: Ulid,

    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

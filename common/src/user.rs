use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u64,

    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

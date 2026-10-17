use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(with = "crate::u64_as_string")]
    pub id: u64,

    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

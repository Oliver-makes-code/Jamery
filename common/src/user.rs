use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub id: Ulid,

    pub username: String,
    pub display_name: Option<String>,
    pub avatar_url: Option<String>,
}

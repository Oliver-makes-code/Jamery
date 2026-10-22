use serde::{Deserialize, Serialize};
use ulid::Ulid;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OAuthArgs {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Claims {
    pub exp: u64,
    pub sub: Ulid,
}

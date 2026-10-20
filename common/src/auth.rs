use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OAuthArgs {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Claims {
    pub exp: u64,
    #[serde(with = "crate::u64_as_string")]
    pub sub: u64,
}

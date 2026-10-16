use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OAuthArgs {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: u64,
    pub sub: String,
}

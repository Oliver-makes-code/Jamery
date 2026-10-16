use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct OAuthArgs {
    pub code: String,
    pub state: String,
}

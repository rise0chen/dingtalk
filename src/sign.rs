use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sign {
    pub access_token: String,
    pub timestamp: Option<u128>,
    pub sign: Option<String>,
}

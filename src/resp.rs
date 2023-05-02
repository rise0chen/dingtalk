use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response {
    pub errcode: i64,
    pub errmsg: String,
}

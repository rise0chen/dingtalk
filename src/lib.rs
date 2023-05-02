pub mod msg;
mod resp;
mod sign;

use base64::{engine::general_purpose, Engine};
use hmac::{Hmac, Mac};
use msg::Message;
use resp::Response;
use sha2::Sha256;
use sign::Sign;
use std::io::{Error, ErrorKind};
use std::time::SystemTime;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;
const DEFAULT_DINGTALK_ROBOT_URL: &str = "https://oapi.dingtalk.com/robot/send";

#[derive(Clone)]
pub struct DingTalk {
    pub webhook_url: String,
    /// webhook中的access_token
    pub access_token: String,
    /// 加密密钥，可空
    pub secret_token: String,
}

impl DingTalk {
    pub fn new(access_token: String, secret_token: String) -> Self {
        DingTalk {
            webhook_url: DEFAULT_DINGTALK_ROBOT_URL.into(),
            access_token,
            secret_token,
        }
    }

    pub fn set_webhook_url(&mut self, webhook_url: String) {
        self.webhook_url = webhook_url;
    }

    pub async fn send(&self, message: Message) -> Result<()> {
        let sign = self.generate_sign()?;
        let client = reqwest::Client::new();
        let response = client.post(&self.webhook_url).query(&sign).json(&message).send().await?;
        let resp: Response = response.json().await?;
        if resp.errcode == 0 {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, resp.errmsg).into())
        }
    }

    pub fn generate_sign(&self) -> Result<Sign> {
        let sign = if !self.secret_token.is_empty() {
            let timestamp = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_millis();
            let timestamp_and_secret = &format!("{}\n{}", timestamp, self.secret_token);
            let hmac_sha256 = calc_hmac_sha256(self.secret_token.as_bytes(), timestamp_and_secret.as_bytes())?;
            let sign = general_purpose::STANDARD.encode(hmac_sha256);
            Sign {
                access_token: self.access_token.clone(),
                timestamp: Some(timestamp),
                sign: Some(sign),
            }
        } else {
            Sign {
                access_token: self.access_token.clone(),
                timestamp: None,
                sign: None,
            }
        };
        Ok(sign)
    }
}

fn calc_hmac_sha256(key: &[u8], message: &[u8]) -> Result<Vec<u8>> {
    let mut mac = Hmac::<Sha256>::new_from_slice(key)?;
    mac.update(message);
    Ok(mac.finalize().into_bytes().to_vec())
}

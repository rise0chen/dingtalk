# DingtalkRobot

钉钉自定义机器人

- 参考文档: <https://open.dingtalk.com/document/robots/custom-robot-access>

## Usage

```rust
use dingtalk_robot::{DingTalk, msg::Message};

#[tokio::main]
pub async fn main() {
    let dt = DingTalk::new("<access_token>", "<secret_token>");
    dt.send(Message::new_text("Hello world!".into())).await.unwrap();
}
```

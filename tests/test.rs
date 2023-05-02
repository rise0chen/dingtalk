use dingtalk_robot::{
    msg::{ActionCardBtn, ActionCardBtnOrientation, FeedCardLink, Message},
    DingTalk,
};
use std::env::var;

#[tokio::test]
async fn test() {
    let access_token = var("ACCESS_TOKEN").unwrap_or_default();
    let secret_token = var("SECRET_TOKEN").unwrap_or_default();
    let robot = DingTalk::new(access_token, secret_token);
    robot.send(Message::new_text("text content".into(), None)).await.unwrap();
    robot
        .send(Message::new_markdown("markdown title".into(), "## markdown content".into(), None))
        .await
        .unwrap();
    robot
        .send(Message::new_link(
            "link title".into(),
            "link text".into(),
            "https://www.baidu.com/favicon.ico".into(),
            "https://www.baidu.com".into(),
        ))
        .await
        .unwrap();
    let btns = vec![
        ActionCardBtn {
            title: "baidu".into(),
            action_url: "https://www.baidu.com".into(),
        },
        ActionCardBtn {
            title: "bing".into(),
            action_url: "https://www.bing.com".into(),
        },
    ];
    robot
        .send(Message::new_single_action_card(
            "single_action title".into(),
            "single_action text".into(),
            btns[0].clone(),
        ))
        .await
        .unwrap();
    robot
        .send(Message::new_multi_action_card(
            "multi_action title".into(),
            "multi_action text".into(),
            ActionCardBtnOrientation::Landscape,
            btns.clone(),
        ))
        .await
        .unwrap();
    robot
        .send(Message::new_multi_action_card(
            "multi_action title".into(),
            "multi_action text".into(),
            ActionCardBtnOrientation::Vertical,
            btns,
        ))
        .await
        .unwrap();
    let links = vec![
        FeedCardLink {
            title: "baidu".into(),
            pic_url: "https://www.baidu.com/favicon.ico".into(),
            message_url: "https://www.baidu.com".into(),
        },
        FeedCardLink {
            title: "bing".into(),
            pic_url: "https://www.bing.com/favicon.ico".into(),
            message_url: "https://www.bing.com".into(),
        },
    ];
    robot.send(Message::new_feed_card(links)).await.unwrap();
}

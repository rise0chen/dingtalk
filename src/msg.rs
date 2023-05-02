use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AtUser {
    pub at_mobiles: Vec<String>,
    pub at_user_ids: Vec<String>,
    pub is_at_all: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextMessage {
    pub content: String,
}
impl TextMessage {
    pub fn new(content: String) -> Self {
        Self { content }
    }
}
impl From<TextMessage> for Message {
    fn from(value: TextMessage) -> Self {
        Self::Text { text: value, at: None }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinkMessage {
    pub title: String,
    pub text: String,
    pub pic_url: String,
    pub message_url: String,
}
impl LinkMessage {
    pub fn new(title: String, text: String, pic_url: String, message_url: String) -> Self {
        Self {
            title,
            text,
            pic_url,
            message_url,
        }
    }
}
impl From<LinkMessage> for Message {
    fn from(value: LinkMessage) -> Self {
        Self::Link { link: value }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkdownMessage {
    pub title: String,
    pub text: String,
}
impl MarkdownMessage {
    pub fn new(title: String, text: String) -> Self {
        Self { title, text }
    }
}
impl From<MarkdownMessage> for Message {
    fn from(value: MarkdownMessage) -> Self {
        Self::Markdown { markdown: value, at: None }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionCardMessage {
    pub title: String,
    pub text: String,
    #[serde(flatten)]
    pub action: ActionCardAction,
}
impl ActionCardMessage {
    pub fn new_single(title: String, text: String, btn: ActionCardBtn) -> Self {
        Self {
            title,
            text,
            action: ActionCardAction::Single {
                single_title: btn.title,
                single_url: btn.action_url,
            },
        }
    }
    pub fn new_multi(title: String, text: String, btn_orientation: ActionCardBtnOrientation, btns: Vec<ActionCardBtn>) -> Self {
        Self {
            title,
            text,
            action: ActionCardAction::Multi { btn_orientation, btns },
        }
    }
}
impl From<ActionCardMessage> for Message {
    fn from(value: ActionCardMessage) -> Self {
        Self::ActionCard { action_card: value }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ActionCardAction {
    Single {
        #[serde(rename = "singleTitle")]
        single_title: String,
        #[serde(rename = "actionURL")]
        single_url: String,
    },
    Multi {
        btn_orientation: ActionCardBtnOrientation,
        btns: Vec<ActionCardBtn>,
    },
}
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub enum ActionCardBtnOrientation {
    #[default]
    #[serde(rename = "0")]
    Vertical,
    #[serde(rename = "1")]
    Landscape,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ActionCardBtn {
    pub title: String,
    #[serde(rename = "actionURL")]
    pub action_url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedCardMessage {
    pub links: Vec<FeedCardLink>,
}
impl FeedCardMessage {
    pub fn new(links: Vec<FeedCardLink>) -> Self {
        Self { links }
    }
}
impl From<FeedCardMessage> for Message {
    fn from(value: FeedCardMessage) -> Self {
        Self::FeedCard { feed_card: value }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FeedCardLink {
    pub title: String,
    #[serde(rename = "messageURL")]
    pub message_url: String,
    #[serde(rename = "picURL")]
    pub pic_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "msgtype", rename_all = "camelCase")]
pub enum Message {
    Text {
        text: TextMessage,
        at: Option<AtUser>,
    },
    Link {
        link: LinkMessage,
    },
    Markdown {
        markdown: MarkdownMessage,
        at: Option<AtUser>,
    },
    ActionCard {
        #[serde(rename = "actionCard")]
        action_card: ActionCardMessage,
    },
    FeedCard {
        #[serde(rename = "feedCard")]
        feed_card: FeedCardMessage,
    },
}
impl Message {
    pub fn new_text(content: String, at: Option<AtUser>) -> Self {
        Self::Text {
            text: TextMessage { content },
            at,
        }
    }

    pub fn new_markdown(title: String, text: String, at: Option<AtUser>) -> Self {
        Self::Markdown {
            markdown: MarkdownMessage { title, text },
            at,
        }
    }

    pub fn new_link(title: String, text: String, pic_url: String, message_url: String) -> Self {
        Self::Link {
            link: LinkMessage {
                title,
                text,
                pic_url,
                message_url,
            },
        }
    }

    pub fn new_single_action_card(title: String, text: String, btn: ActionCardBtn) -> Self {
        ActionCardMessage::new_single(title, text, btn).into()
    }

    pub fn new_multi_action_card(title: String, text: String, btn_orientation: ActionCardBtnOrientation, btns: Vec<ActionCardBtn>) -> Self {
        ActionCardMessage::new_multi(title, text, btn_orientation, btns).into()
    }

    pub fn new_feed_card(links: Vec<FeedCardLink>) -> Self {
        Self::FeedCard {
            feed_card: FeedCardMessage { links },
        }
    }
}

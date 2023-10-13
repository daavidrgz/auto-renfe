use serde::{Deserialize, Serialize};
use teloxide::types::ChatId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    id: UserId,
    username: String,
    password: String,
}

impl User {
    pub fn new<T>(id: T, username: String, password: String) -> Self
    where
        T: Into<UserId>,
    {
        Self {
            id: id.into(),
            username,
            password,
        }
    }
    pub fn chat_id(&self) -> ChatId {
        ChatId::from(self.id)
    }
    pub fn id(&self) -> UserId {
        self.id
    }
    pub fn username(&self) -> &str {
        &self.username
    }
    pub fn password(&self) -> &str {
        &self.password
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct UserId(pub i64);
impl From<ChatId> for UserId {
    fn from(id: ChatId) -> Self {
        Self(id.0)
    }
}
impl From<UserId> for ChatId {
    fn from(id: UserId) -> Self {
        Self(id.0)
    }
}
impl From<i64> for UserId {
    fn from(id: i64) -> Self {
        Self(id)
    }
}
impl From<UserId> for i64 {
    fn from(id: UserId) -> Self {
        id.0
    }
}

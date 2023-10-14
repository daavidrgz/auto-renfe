use crate::infrastructure::repositories::user_repository::UserRepository;

use super::dialogues::account::login::LoginDialogueState;
use super::dialogues::purchase::PurchaseDialogueState;
use super::{MyDialogue, MyHandler, MyHandlerResult};
use anyhow::anyhow;
use async_trait::async_trait;
use dptree::entry;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

#[async_trait]
pub trait Menu
where
    Self: DeserializeOwned + Send + Clone + Sync + 'static + Into<MenuEvent>,
    MenuEvent: From<Self>,
{
    fn menu_keyboard() -> InlineKeyboardMarkup {
        let menu_items = Self::menu_items().iter().map(|item| {
            let button_name = item.0;
            let event = &item.1;
            let menu_event = MenuEvent::from(event.clone());

            [InlineKeyboardButton::callback(
                button_name,
                serde_json::to_string(&menu_event).unwrap(),
            )]
        });
        InlineKeyboardMarkup::new(menu_items)
    }

    async fn show_menu(bot: Bot, msg: Message) -> MyHandlerResult {
        // Msg is the message that originated the action
        bot.send_message(msg.chat.id, "Select an option:")
            .reply_markup(Self::menu_keyboard())
            .await?;
        Ok(())
    }

    async fn show_menu_in_message(bot: Bot, msg: Message) -> MyHandlerResult {
        bot.edit_message_reply_markup(msg.chat.id, msg.id)
            .reply_markup(Self::menu_keyboard())
            .await?;
        Ok(())
    }

    fn menu_items<'a>() -> &'a [(&'a str, Self)];
    async fn handle(&self, bot: Bot, q: CallbackQuery, dialogue: MyDialogue) -> MyHandlerResult;
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum MenuEvent {
    MainMenuEvent(MainMenuEvent),
    AccountMenuEvent(AccountMenuEvent),
}

impl MenuEvent {
    pub async fn handle(self, bot: Bot, q: CallbackQuery, dialogue: MyDialogue) -> MyHandlerResult {
        let bot_clone = bot.clone();
        let q_id = q.id.clone();
        let result = match self {
            MenuEvent::MainMenuEvent(event) => event.handle(bot, q, dialogue).await,
            MenuEvent::AccountMenuEvent(event) => event.handle(bot, q, dialogue).await,
        };
        bot_clone.answer_callback_query(q_id).await?;
        result
    }

    pub fn schema() -> MyHandler {
        entry()
            .filter_map(|q: CallbackQuery| {
                let data = q.data?;
                let event = serde_json::from_str::<Self>(&data);
                event.ok()
            })
            .endpoint(Self::handle)
    }
}

impl From<MainMenuEvent> for MenuEvent {
    fn from(event: MainMenuEvent) -> Self {
        Self::MainMenuEvent(event)
    }
}

impl From<AccountMenuEvent> for MenuEvent {
    fn from(event: AccountMenuEvent) -> Self {
        Self::AccountMenuEvent(event)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum MainMenuEvent {
    OpenAccountMenu,
    OpenPurchaseMenu,
}

#[async_trait]
impl Menu for MainMenuEvent {
    fn menu_items<'a>() -> &'a [(&'a str, Self)] {
        &[
            ("Account", Self::OpenAccountMenu),
            ("Purchase", Self::OpenPurchaseMenu),
        ]
    }

    async fn handle(&self, bot: Bot, q: CallbackQuery, dialogue: MyDialogue) -> MyHandlerResult {
        let original_message = q.message.ok_or(anyhow!("No message"))?;
        match self {
            Self::OpenAccountMenu => {
                AccountMenuEvent::show_menu_in_message(bot, original_message).await?;
            }
            Self::OpenPurchaseMenu => {
                PurchaseDialogueState::start(bot, dialogue, original_message).await?
            }
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Copy)]
pub enum AccountMenuEvent {
    Login,
    Info,
    Back,
}

#[async_trait]
impl Menu for AccountMenuEvent {
    fn menu_items<'a>() -> &'a [(&'a str, Self)] {
        &[
            ("Login", Self::Login),
            ("Info", Self::Info),
            ("Back", Self::Back),
        ]
    }
    async fn handle(&self, bot: Bot, q: CallbackQuery, dialogue: MyDialogue) -> MyHandlerResult {
        let original_message = q.message.ok_or(anyhow!("No message"))?;
        let chat_id = original_message.chat.id;
        match self {
            Self::Login => {
                LoginDialogueState::start(bot, dialogue, original_message).await?;
            }
            Self::Info => {
                let repository = UserRepository::instance().await;
                let Some(user) = repository.find_by_id(chat_id.into()).await? else {
                    bot.send_message(chat_id, "You are not logged in").await?;
                    return Ok(());
                };
                let message = format!("You are logged in as {}", user.username());
                bot.send_message(chat_id, message).await?;
            }
            Self::Back => {
                MainMenuEvent::show_menu_in_message(bot, original_message).await?;
            }
        }
        Ok(())
    }
}

use super::{HandlerResult, MyDialogue};
use async_trait::async_trait;
use dptree::{entry, filter_map};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_prefix::prefix_all;
use teloxide::dispatching::DpHandlerDescription;
use teloxide::prelude::*;
use teloxide::types::{Chat, InlineKeyboardButton, InlineKeyboardMarkup};

#[async_trait]
pub trait Menu
where
    Self: 'static,
{
    type Event: Serialize + DeserializeOwned + Clone + Send + Sync + 'static;
    fn menu_keyboard() -> InlineKeyboardMarkup {
        let menu_items = Self::menu_items().iter().map(|item| {
            [InlineKeyboardButton::callback(
                item.0,
                serde_json::to_string(&item.1).unwrap(),
            )]
        });
        InlineKeyboardMarkup::new(menu_items)
    }

    fn filter_event() -> Handler<'static, DependencyMap, HandlerResult, DpHandlerDescription> {
        filter_map(|q: CallbackQuery| {
            let data = q.data?;
            let event = serde_json::from_str::<Self::Event>(&data);
            event.ok()
        })
        .endpoint(Self::handle)
    }

    async fn show_menu(bot: Bot, msg: Message) -> HandlerResult {
        // Msg is the message that originated the action
        bot.send_message(msg.chat.id, "Select an option:")
            .reply_markup(Self::menu_keyboard())
            .await?;
        Ok(())
    }

    async fn show_menu_in_message(bot: Bot, msg: Message) -> HandlerResult {
        bot.edit_message_reply_markup(msg.chat.id, msg.id)
            .reply_markup(Self::menu_keyboard())
            .await?;
        Ok(())
    }

    fn menu_items<'a>() -> &'a [(&'a str, Self::Event)];
    async fn handle(bot: Bot, q: CallbackQuery, event: Self::Event) -> HandlerResult;
}

pub struct MainMenu {}
#[derive(Serialize, Deserialize, Clone, Copy)]
#[prefix_all("MainMenuEvent_")]
pub enum MainMenuEvent {
    OpenAccountMenu,
    OpenPurchaseMenu,
}

pub struct AccountMenu {}
#[derive(Serialize, Deserialize, Clone, Copy)]
#[prefix_all("AccountMenuEvent_")]
pub enum AccountMenuEvent {
    Login,
    Info,
    Back,
}

pub fn schema() -> Handler<'static, DependencyMap, HandlerResult, DpHandlerDescription> {
    entry()
        .branch(MainMenu::filter_event())
        .branch(AccountMenu::filter_event())
}

#[async_trait]
impl Menu for MainMenu {
    type Event = MainMenuEvent;

    fn menu_items<'a>() -> &'a [(&'a str, Self::Event)] {
        &[
            ("Account", MainMenuEvent::OpenAccountMenu),
            ("Purchase", MainMenuEvent::OpenPurchaseMenu),
        ]
    }

    async fn handle(bot: Bot, q: CallbackQuery, event: Self::Event) -> HandlerResult {
        match event {
            MainMenuEvent::OpenAccountMenu => {
                AccountMenu::show_menu_in_message(bot, q.message.unwrap()).await?;
            }
            MainMenuEvent::OpenPurchaseMenu => {
                bot.send_message(q.message.unwrap().chat.id, "Purchase menu")
                    .await?;
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Menu for AccountMenu {
    type Event = AccountMenuEvent;
    fn menu_items<'a>() -> &'a [(&'a str, Self::Event)] {
        &[
            ("Login", AccountMenuEvent::Login),
            ("Info", AccountMenuEvent::Info),
            ("Back", AccountMenuEvent::Back),
        ]
    }
    async fn handle(bot: Bot, q: CallbackQuery, event: Self::Event) -> HandlerResult {
        match event {
            AccountMenuEvent::Login => {
                bot.send_message(q.message.unwrap().chat.id, "Login menu")
                    .await?;
            }
            AccountMenuEvent::Info => {
                bot.send_message(q.message.unwrap().chat.id, "Info menu")
                    .await?;
            }
            AccountMenuEvent::Back => {
                MainMenu::show_menu_in_message(bot, q.message.unwrap()).await?;
            }
        }
        Ok(())
    }
}

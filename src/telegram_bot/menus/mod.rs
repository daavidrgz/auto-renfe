use super::{HandlerResult, MyDialogue};
use dptree::{entry, filter};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_prefix::prefix_all;
use teloxide::dispatching::DpHandlerDescription;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

pub async fn show_menu<T: Menu>(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Select an option:")
        .reply_markup(T::menu_keyboard())
        .await?;
    Ok(())
}

pub async fn show_menu_in_message<T: Menu>(bot: Bot, msg: Message) -> HandlerResult {
    bot.edit_message_reply_markup(msg.chat.id, msg.id)
        .reply_markup(T::menu_keyboard())
        .await?;
    Ok(())
}

pub trait Menu {
    type Event: Serialize + DeserializeOwned;
    fn menu_keyboard() -> InlineKeyboardMarkup {
        let menu_items = Self::menu_items().iter().map(|item| {
            [InlineKeyboardButton::callback(
                item.0,
                serde_json::to_string(&item.1).unwrap(),
            )]
        });
        InlineKeyboardMarkup::new(menu_items)
    }

    fn filter_event<'a>() -> Handler<'a, DependencyMap, HandlerResult, DpHandlerDescription> {
        filter(move |q: CallbackQuery| {
            q.data.is_some_and(move |data| {
                let data: Result<Self::Event, _> = serde_json::from_str(&data);
                data.is_ok()
            })
        })
        .endpoint(|bot: Bot, msg: Message| async move { Self::handle(bot, msg) })
    }
    fn menu_items<'a>() -> &'a [(&'a str, Self::Event)];
    fn handle(bot: Bot, msg: Message) -> HandlerResult;
}

pub struct MainMenu {}
#[derive(Serialize, Deserialize)]
#[prefix_all("MainMenuEvent_")]
pub enum MainMenuEvent {
    OpenAccountMenu,
    OpenPurchaseMenu,
}

pub struct AccountMenu {}
#[derive(Serialize, Deserialize)]
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

impl Menu for MainMenu {
    type Event = MainMenuEvent;
    fn handle(bot: Bot, msg: Message) -> HandlerResult {
        todo!()
    }
    fn menu_items<'a>() -> &'a [(&'a str, Self::Event)] {
        &[
            ("Account", MainMenuEvent::OpenAccountMenu),
            ("Purchase", MainMenuEvent::OpenPurchaseMenu),
        ]
    }
}

impl Menu for AccountMenu {
    type Event = AccountMenuEvent;
    fn handle(bot: Bot, msg: Message) -> HandlerResult {
        todo!()
    }
    fn menu_items<'a>() -> &'a [(&'a str, Self::Event)] {
        &[
            ("Login", AccountMenuEvent::Login),
            ("Info", AccountMenuEvent::Info),
            ("Back", AccountMenuEvent::Back),
        ]
    }
}

// async fn go_back_menu(bot: Bot, msg: Message) -> HandlerResult {
//     bot.edit_message_reply_markup(msg.chat.id, msg.id)
//         .reply_markup(menu_keyboard())
//         .await?;
//     Ok(())
// }

// async fn handle_menu_event(bot: Bot, q: CallbackQuery) -> HandlerResult {
//     let item = q.data.ok_or("No data")?;
//     let event = serde_json::from_str::<MainMenuEvent>(&item)?;
//     match event {
//         MainMenuEvent::OpenAccountMenu => {
//             show_account_menu(bot, q.message.unwrap()).await?;
//         }
//         MainMenuEvent::OpenPurchaseMenu => {
//             bot.send_message(q.message.unwrap().chat.id, "Purchase menu")
//                 .await?;
//         }
//     }
//     Ok(())
// }

// async fn show_account_menu(bot: Bot, msg: Message) -> HandlerResult {
//     bot.edit_message_reply_markup(msg.chat.id, msg.id)
//         .reply_markup(account_menu_keyboard())
//         .await?;
//     Ok(())
// }

// async fn handle_account_menu_event(bot: Bot, q: CallbackQuery) -> HandlerResult {
//     let data = q.data.ok_or("No data")?;
//     let event = serde_json::from_str::<AccountMenuEvent>(&data)?;
//     match event {
//         AccountMenuEvent::Login => {
//             bot.send_message(q.message.unwrap().chat.id, "Login menu")
//                 .await?;
//         }
//         AccountMenuEvent::Info => {
//             bot.send_message(q.message.unwrap().chat.id, "Info menu")
//                 .await?;
//         }
//         AccountMenuEvent::Back => {
//             go_back_menu(bot, q.message.unwrap()).await?;
//         }
//     }
//     Ok(())
// }

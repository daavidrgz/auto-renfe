use super::{HandlerResult, MyDialogue};
use dptree::{entry, filter};
use teloxide::dispatching::DpHandlerDescription;
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

fn filter_menu(
    prefix: &'static str,
) -> Handler<'_, DependencyMap, HandlerResult, DpHandlerDescription> {
    filter(move |q: CallbackQuery| q.data.is_some_and(|data| data.starts_with(prefix)))
}

pub fn schema() -> Handler<'static, DependencyMap, HandlerResult, DpHandlerDescription> {
    entry()
        .branch(filter_menu("main_menu_").endpoint(navigate_menu))
        .branch(filter_menu("account_menu_").endpoint(navigate_account_menu))
}
fn menu_keyboard() -> InlineKeyboardMarkup {
    let menu_items = ["Account", "Purchase"];
    let menu_items = menu_items.map(|item| {
        [InlineKeyboardButton::callback(
            item,
            format!("main_menu_{item}"),
        )]
    });
    InlineKeyboardMarkup::new(menu_items)
}

pub async fn show_menu(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Select an option:")
        .reply_markup(menu_keyboard())
        .await?;
    Ok(())
}

async fn go_back_menu(bot: Bot, msg: Message) -> HandlerResult {
    bot.edit_message_reply_markup(msg.chat.id, msg.id)
        .reply_markup(menu_keyboard())
        .await?;
    Ok(())
}

async fn navigate_menu(bot: Bot, q: CallbackQuery) -> HandlerResult {
    if let Some(item) = q.data {
        match item.as_str() {
            "main_menu_Account" => {
                show_account_menu(bot, q.message.unwrap()).await?;
            }
            "main_menu_Purchase" => {
                bot.send_message(q.message.unwrap().chat.id, "Purchase menu")
                    .await?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn account_menu_keyboard() -> InlineKeyboardMarkup {
    let menu_items = ["Login", "Info", "Back"];
    let menu_items = menu_items.map(|item| {
        [InlineKeyboardButton::callback(
            item,
            format!("account_menu_{item}"),
        )]
    });
    InlineKeyboardMarkup::new(menu_items)
}
async fn show_account_menu(bot: Bot, msg: Message) -> HandlerResult {
    bot.edit_message_reply_markup(msg.chat.id, msg.id)
        .reply_markup(account_menu_keyboard())
        .await?;
    Ok(())
}

async fn navigate_account_menu(bot: Bot, dialogue: MyDialogue, q: CallbackQuery) -> HandlerResult {
    if let Some(item) = &q.data {
        match item.as_str() {
            "account_menu_Login" => {
                bot.send_message(dialogue.chat_id(), "Login menu").await?;
            }
            "account_menu_Info" => {
                bot.send_message(dialogue.chat_id(), "Info menu").await?;
            }
            "account_menu_Back" => {
                go_back_menu(bot, q.message.unwrap()).await?;
            }
            _ => {}
        }
    }
    Ok(())
}

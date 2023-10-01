use std::error::Error;

use dptree::{case, deps, filter};

use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, DpHandlerDescription, UpdateHandler},
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup},
    utils::command::BotCommands,
};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

#[derive(Clone)]
pub struct AutoRenfeBot {
    bot: Bot,
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Menu,
}

impl Default for AutoRenfeBot {
    fn default() -> Self {
        Self::new()
    }
}
impl AutoRenfeBot {
    pub fn new() -> Self {
        Self {
            bot: Bot::from_env(),
        }
    }

    pub async fn run(self) {
        pretty_env_logger::init();
        log::info!("Starting renfebot...");

        Dispatcher::builder(self.bot, schema())
            .dependencies(deps![InMemStorage::<State>::new()])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }
}

/// These commands are supported:
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase")]
enum Command {
    /// Display this text.
    Help,
    /// Start the purchase procedure.
    Menu,
    /// Cancel the purchase procedure.
    Cancel,
}

fn filter_menu(
    prefix: &'static str,
) -> Handler<'_, DependencyMap, HandlerResult, DpHandlerDescription> {
    filter(move |q: CallbackQuery| q.data.is_some_and(|data| data.starts_with(prefix)))
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![State::Menu]
                .branch(case![Command::Help].endpoint(help))
                .branch(case![Command::Menu].endpoint(show_menu)),
        )
        .branch(case![Command::Cancel].endpoint(cancel));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        // .branch(case![State::ReceiveFullName].endpoint(receive_full_name))
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler = Update::filter_callback_query()
        .branch(filter_menu("main_menu").endpoint(navigate_menu))
        .branch(filter_menu("account_menu").endpoint(navigate_account_menu));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

// async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
//     bot.send_message(msg.chat.id, "Let's start! What's your full name?")
//         .await?;
//     dialogue.update(State::ReceiveFullName).await?;
//     Ok(())
// }

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
async fn show_menu(bot: Bot, msg: Message) -> HandlerResult {
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

async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Cancelling the dialogue.")
        .await?;
    dialogue.exit().await?;
    Ok(())
}

async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(
        msg.chat.id,
        "Unable to handle the message. Type /help to see the usage.",
    )
    .await?;
    Ok(())
}

// async fn receive_full_name(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
//     match msg.text().map(ToOwned::to_owned) {
//         Some(full_name) => {
//             let products = ["Apple", "Banana", "Orange", "Potato"]
//                 .map(|product| InlineKeyboardButton::callback(product, product));

//             bot.send_message(msg.chat.id, "Select a product:")
//                 .reply_markup(InlineKeyboardMarkup::new([products]))
//                 .await?;
//             dialogue
//                 .update(State::ReceiveProductChoice { full_name })
//                 .await?;
//         }
//         None => {
//             bot.send_message(msg.chat.id, "Please, send me your full name.")
//                 .await?;
//         }
//     }

//     Ok(())
// }

// async fn receive_product_selection(
//     bot: Bot,
//     dialogue: MyDialogue,
//     full_name: String, // Available from `State::ReceiveProductChoice`.
//     q: CallbackQuery,
// ) -> HandlerResult {
//     if let Some(product) = &q.data {
//         bot.send_message(
//             dialogue.chat_id(),
//             format!("{full_name}, product '{product}' has been purchased successfully!"),
//         )
//         .await?;
//         dialogue.exit().await?;
//     }

//     Ok(())
// }

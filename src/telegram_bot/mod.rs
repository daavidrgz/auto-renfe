use self::{dialogues::DialogueState, menus::MainMenuEvent};
use crate::Result;
use dptree::{case, deps};
use menus::{Menu, MenuEvent};
use std::error::Error;
use teloxide::{
    dispatching::{dialogue, dialogue::InMemStorage, DpHandlerDescription, UpdateHandler},
    prelude::*,
    utils::command::BotCommands,
};

type MyDialogue = Dialogue<GlobalState, InMemStorage<GlobalState>>;
type MyHandlerResult = Result<()>;
type MyHandler = Handler<'static, DependencyMap, MyHandlerResult, DpHandlerDescription>;

pub mod dialogues;
pub mod menus;
#[derive(Clone)]
pub struct AutoRenfeBot {
    bot: Bot,
}

#[derive(Clone, Default)]
pub enum GlobalState {
    #[default]
    Start,
    InDialogue(DialogueState),
}

impl<T> From<T> for GlobalState
where
    T: Into<DialogueState>,
{
    fn from(state: T) -> Self {
        Self::InDialogue(state.into())
    }
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
            .dependencies(deps![InMemStorage::<GlobalState>::new()])
            .enable_ctrlc_handler()
            .build()
            // TODO: use dispatch_with_listener
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

// TODO: hacer esto con un enum que represente las opciones de un menu
// e intentar parsear...

fn schema() -> UpdateHandler<Box<dyn Error + Send + Sync + 'static>> {
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![GlobalState::Start]
                .branch(case![Command::Help].endpoint(help))
                .branch(case![Command::Menu].endpoint(MainMenuEvent::show_menu)),
        )
        .branch(case![Command::Cancel].endpoint(cancel));

    let dialogue_handler = case![GlobalState::InDialogue(dialogue)].chain(DialogueState::schema());

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(dialogue_handler)
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler = Update::filter_callback_query().branch(MenuEvent::schema());

    dialogue::enter::<Update, InMemStorage<GlobalState>, GlobalState, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

async fn help(bot: Bot, msg: Message) -> MyHandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string())
        .await?;
    Ok(())
}

async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> MyHandlerResult {
    bot.send_message(msg.chat.id, "Cancelling the dialogue.")
        .await?;
    dialogue.exit().await?;
    Ok(())
}

async fn invalid_state(bot: Bot, msg: Message) -> MyHandlerResult {
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

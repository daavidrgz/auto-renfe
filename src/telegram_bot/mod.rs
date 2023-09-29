use std::error::Error;

use dptree::{case, deps};
use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

type MyDialogue = Dialogue<State, InMemStorage<State>>;
type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;
pub struct AutoRenfeBot {
    bot: Bot,
}

#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveFullName,
    ReceiveAge {
        full_name: String,
    },
    ReceiveLocation {
        full_name: String,
        age: u8,
    },
}

impl AutoRenfeBot {
    pub fn new(token: &str) -> Self {
        Self {
            bot: Bot::new(token),
        }
    }

    pub async fn run(self) {
        pretty_env_logger::init();
        log::info!("Starting renfebot...");

        let handler = Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(case![State::Start].endpoint(start))
            .branch(case![State::ReceiveFullName].endpoint(receive_full_name))
            .branch(case![State::ReceiveAge { full_name }].endpoint(receive_age))
            .branch(case![State::ReceiveLocation { full_name, age }].endpoint(receive_location));

        Dispatcher::builder(self.bot, handler)
            .dependencies(deps![InMemStorage::<State>::new()])
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await
    }
}

pub async fn start(bot: Bot, dialogue: MyDialogue, message: Message) -> HandlerResult {
    bot.send_message(message.chat.id, "Hello! what is your name?")
        .await?;
    dialogue.update(State::ReceiveFullName).await?;
    Ok(())
}

pub async fn receive_full_name(bot: Bot, dialogue: MyDialogue, message: Message) -> HandlerResult {
    match message.text() {
        Some(text) => {
            bot.send_message(message.chat.id, "How old are you?")
                .await?;
            dialogue
                .update(State::ReceiveAge {
                    full_name: text.to_string(),
                })
                .await?;
        }
        None => {
            bot.send_message(message.chat.id, "Please, send me a text")
                .await?;
        }
    }
    Ok(())
}

pub async fn receive_age(
    bot: Bot,
    dialogue: MyDialogue,
    full_name: String,
    message: Message,
) -> HandlerResult {
    match message.text().map(|text| text.parse::<u8>()) {
        Some(Ok(age)) => {
            bot.send_message(message.chat.id, "Where are you from?")
                .await?;
            dialogue
                .update(State::ReceiveLocation { full_name, age })
                .await?;
        }
        _ => {
            bot.send_message(message.chat.id, "Please, send me a number")
                .await?;
        }
    }
    Ok(())
}

pub async fn receive_location(
    bot: Bot,
    dialogue: MyDialogue,
    (full_name, age): (String, u8),
    message: Message,
) -> HandlerResult {
    match message.text() {
        Some(location) => {
            bot.send_message(
                message.chat.id,
                format!(
                    "Your name is {}, you are {} years old and you are from {}",
                    full_name, age, location
                ),
            )
            .await?;
            dialogue.exit().await?;
        }
        None => {
            bot.send_message(message.chat.id, "Please, send me a text")
                .await?;
        }
    }
    Ok(())
}

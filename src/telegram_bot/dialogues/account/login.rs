use dptree::{case, entry};
use teloxide::{prelude::*, types::Message, Bot};

use crate::telegram_bot::{MyDialogue, MyHandler, MyHandlerResult};

#[derive(Clone, Debug)]
pub enum LoginDialogueState {
    ReceiveUsername,
    ReceivePassword { username: String },
}

impl LoginDialogueState {
    pub fn schema() -> MyHandler {
        entry()
            .branch(case![LoginDialogueState::ReceiveUsername].endpoint(Self::receive_username))
            .branch(
                case![LoginDialogueState::ReceivePassword { username }]
                    .endpoint(Self::receive_password),
            )
    }

    pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> MyHandlerResult {
        bot.send_message(
            msg.chat.id,
            "Login procedure started. Type /cancel to cancel the procedure.\nPlease, send me your username",
        )
        .await?;
        dialogue.update(LoginDialogueState::ReceiveUsername).await?;
        Ok(())
    }

    async fn receive_username(bot: Bot, dialogue: MyDialogue, msg: Message) -> MyHandlerResult {
        let username = match msg.text() {
            Some(username) => username.to_string(),
            None => {
                bot.send_message(msg.chat.id, "Please, send me your username")
                    .await?;
                return Ok(());
            }
        };
        bot.send_message(msg.chat.id, "Please, send me your password")
            .await?;
        dialogue
            .update(LoginDialogueState::ReceivePassword { username })
            .await?;

        Ok(())
    }
    async fn receive_password(
        bot: Bot,
        dialogue: MyDialogue,
        username: String,
        msg: Message,
    ) -> MyHandlerResult {
        let password = match msg.text() {
            Some(password) => password.to_string(),
            None => {
                bot.send_message(msg.chat.id, "Please, send me your password")
                    .await?;
                return Ok(());
            }
        };
        // TODO: delete msg for password privacy
        let message = format!("You are logged in with user {username} and password {password}");
        bot.send_message(msg.chat.id, message).await?;
        dialogue.exit().await?;

        Ok(())
    }
}

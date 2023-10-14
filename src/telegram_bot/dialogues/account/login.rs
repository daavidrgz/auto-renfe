use dptree::{case, entry};
use teloxide::{prelude::*, types::Message, Bot};

use crate::{
    entities::user::User,
    infrastructure::repositories::user_repository::UserRepository,
    telegram_bot::{MyDialogue, MyHandler, MyHandlerResult},
};

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
            "Login procedure started. Type /cancel to cancel the procedure.",
        )
        .await?;
        bot.send_message(msg.chat.id, "Please, send me your username")
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
        let chat_id = msg.chat.id;
        let password = match msg.text() {
            Some(password) => password.to_string(),
            None => {
                bot.send_message(chat_id, "Please, send me your password")
                    .await?;
                return Ok(());
            }
        };
        bot.delete_message(chat_id, msg.id).await?;
        let message = format!(
            "You are logged in with username '{username}'. Password has been deleted for privacy."
        );
        bot.send_message(chat_id, message).await?;
        dialogue.exit().await?;

        let repository = UserRepository::instance().await;
        let user = User::new(chat_id, username, password);
        repository.add_or_update(user).await?;

        Ok(())
    }
}

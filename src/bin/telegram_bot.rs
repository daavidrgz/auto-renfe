use auto_renfe::telegram_bot::AutoRenfeBot;
use dotenv_codegen::dotenv;

#[tokio::main]
async fn main() {
    // load environment variables from .env file

    let token = dotenv!("TELOXIDE_TOKEN");

    let bot = AutoRenfeBot::new(token);
    bot.run().await;
}

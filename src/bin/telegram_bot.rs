use auto_renfe::telegram_bot::AutoRenfeBot;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    // load environment variables from .env file
    dotenv().expect("failed to load .env file");

    let bot = AutoRenfeBot::new();
    bot.run().await;
}

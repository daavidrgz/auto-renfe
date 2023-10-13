use std::error::Error;

pub mod entities;
pub mod infrastructure;
pub mod telegram_bot;

type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

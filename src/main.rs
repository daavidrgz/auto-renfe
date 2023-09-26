use scraper::RenfeScraper;

pub mod scraper;

#[tokio::main]
async fn main() {
    let mut scraper = RenfeScraper::new().await;
    scraper.find_trains("Madrid", "Barcelona").await;
    scraper.close().await;
}

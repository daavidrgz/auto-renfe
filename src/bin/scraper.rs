use auto_renfe::scraper::RenfeScraper;

#[tokio::main]
async fn main() {
    println!("scraper 1");
    let mut scraper = RenfeScraper::new().await;
    println!("scraper 2");
    let mut scraper2 = RenfeScraper::new().await;
    println!("scraper 3");
    let mut scraper3 = RenfeScraper::new().await;

    scraper.find_trains("Madrid", "Barcelona").await;
    scraper2.find_trains("Madrid", "Barcelona").await;
    scraper3.find_trains("Madrid", "Barcelona").await;
    scraper.close().await;
    scraper2.close().await;
    scraper3.close().await;
}

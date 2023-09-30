use auto_renfe::scraper::{constants::SearchFilterBuilder, RenfeScraper};

#[tokio::main]
async fn main() {
    let mut scraper = RenfeScraper::new().await.expect("failed to create scraper");
    let search_filters = SearchFilterBuilder::default()
        .origin("SANTIAGO")
        .destination("PONTEVEDRA")
        .departure_date("30/10/2023")
        .build()
        .unwrap();
    let _ = scraper.find_trains(&search_filters).await;
    // scraper.close().await;
}

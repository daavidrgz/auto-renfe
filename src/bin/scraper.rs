use auto_renfe::infrastructure::scraper::{constants::SearchFilterBuilder, RenfeScraper};

#[tokio::main]
async fn main() {
    let mut scraper = RenfeScraper::new().await.expect("failed to create scraper");
    let search_filters = SearchFilterBuilder::default()
        .origin("SANTIAGO")
        .destination("PONTEVEDRA")
        .departure_date("30/10/2023")
        .min_departure_hour("16:40")
        .max_departure_hour("20:00")
        .build()
        .unwrap();
    let _ = scraper.buy_tickets(&search_filters).await;
    // scraper.close().await;
}

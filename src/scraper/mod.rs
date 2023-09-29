pub mod constants;

use std::error::Error;

use constants::*;
use fantoccini::{Client, ClientBuilder, Locator};

pub struct RenfeScraper {
    client: Client,
}

impl RenfeScraper {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let geckodriver_url = "http://localhost:4444/";
        let capabilities = serde_json::json!({
            "browserName": "chrome",
            "goog:chromeOptions": {
                "args": [
                    "--headless",
                    "--disable-gpu",
                    "--no-sandbox",
                    "--disable-dev-shm-usage"
                ]
            }
        });
        let c = ClientBuilder::native()
            .capabilities(capabilities.as_object().unwrap().clone())
            .connect(geckodriver_url)
            .await?;
        Ok(Self { client: c })
    }

    pub async fn find_trains(&mut self, search_filters: &SearchFilter<'_>) -> Result<(), Box<dyn Error>> {
        self.client.goto(RENFE_URL).await?;
        self.search_stations(search_filters).await?;
        Ok(())
    }
    
    pub async fn search_stations(&mut self, search_filters: &SearchFilter<'_>) -> Result<(), Box<dyn Error>> {
        let origin_locator = Locator::Css("input#origin");
        let destination_locator = Locator::Css("input#destination");
        
        let origin_element = self.client.wait().for_element(origin_locator).await?;
        origin_element.click().await?;
        origin_element.send_keys(search_filters.get_origin()).await?;
        origin_element.send_keys("").await?;

        self.client
            .wait()
            .for_element(destination_locator)
            .await?
            .send_keys(search_filters.get_destination())
            .await?;
        Ok(())
    }

    // pub async fn search_dates(&mut self, search_filters: &SearchFilters) -> Result<(), Err> {

    // }

    pub async fn close(self) {
        self.client.close().await.expect("failed to close browser");
    }
}

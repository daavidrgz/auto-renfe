pub mod constants;

use std::error::Error;

use constants::*;
use fantoccini::{
    actions::{self, ActionSequence, InputSource, KeyAction, KeyActions},
    key::Key,
    Client, ClientBuilder, Locator,
};

pub struct RenfeScraper {
    client: Client,
}

impl RenfeScraper {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let chromedriver_url = "http://localhost:9515/";
        let capabilities = serde_json::json!({
            "browserName": "chrome",
            "goog:chromeOptions": {
                "args": [
                    // "--headless",
                    // "--disable-gpu",
                    // "--no-sandbox",
                    // "--disable-dev-shm-usage"
                ]
            }
        });
        let c = ClientBuilder::native()
            .capabilities(capabilities.as_object().unwrap().clone())
            .connect(chromedriver_url)
            .await?;
        Ok(Self { client: c })
    }

    pub async fn find_trains(
        &mut self,
        search_filters: &SearchFilter<'_>,
    ) -> Result<(), Box<dyn Error>> {
        self.client.goto(RENFE_URL).await?;
        self.search_stations(search_filters).await?;
        Ok(())
    }

    pub async fn search_stations(
        &mut self,
        search_filters: &SearchFilter<'_>,
    ) -> Result<(), Box<dyn Error>> {
        let origin_locator = Locator::Css("input#origin");
        let destination_locator = Locator::Css("input#destination");

        let origin_element = self.client.wait().for_element(origin_locator).await?;
        origin_element.click().await?;
        origin_element
            .send_keys(search_filters.get_origin())
            .await?;

        let key_actions = KeyActions::new("keys".to_string())
            .then(KeyAction::Down {
                value: Key::Down.into(),
            })
            .then(KeyAction::Up {
                value: Key::Down.into(),
            })
            .then(KeyAction::Down {
                value: Key::Enter.into(),
            })
            .then(KeyAction::Up {
                value: Key::Enter.into(),
            });

        self.client.perform_actions(key_actions).await?;

        // Perform an action to simulate down and enter key
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

    async fn press_keys(&mut self, keys: Vec<Key>) -> Result<(), Box<dyn Error>> {
        keys.iter()
            .map(|key| KeyAction::from(KeyAction::Down { value: key.into() }));

        let key_actions = KeyActions::new("keys".to_string());
        for key in keys {
            key_actions.then(KeyAction::Down { value: key.into() });
            key_actions.then(KeyAction::Up { value: key.into() });
        }
        Ok(())
    }
}

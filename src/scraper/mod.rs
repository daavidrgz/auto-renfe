pub mod constants;

use std::error::Error;

use constants::*;
use fantoccini::{
    actions::{InputSource, KeyAction, KeyActions},
    key::Key,
    Client, ClientBuilder, Locator,
};
use futures::future::*;

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
        // Type the origin station
        self.send_keys_by_locator(Locator::Css("input#origin"), search_filters.get_origin())
            .await?;
        // Select the first option and press enter
        self.press_keys(vec![Key::Down, Key::Enter]).await?;
        // Type the destination station
        self.send_keys_by_locator(
            Locator::Css("input#destination"),
            search_filters.get_destination(),
        )
        .await?;
        // Select the first option and press enter
        self.press_keys(vec![Key::Down, Key::Enter]).await?;

        // Click on the "sólo ida" or "ida y vuelta" button
        self.click_element_by_locator(Locator::Css("button.menu-button"))
            .await?;
        // Click on the "sólo ida" button
        self.click_element_by_locator(Locator::Css("button.rf-select__list-text"))
            .await?;

        // Search
        self.click_element_by_locator(Locator::Css("button[title=\"Buscar billete\"]"))
            .await?;

        // Type the departure date
        self.send_keys_by_locator(
            Locator::Css("input#fechaSeleccionada0"),
            search_filters.get_departure_date(),
        )
        .await?;

        // Wait for a train row to appear
        self.client
            .wait()
            .for_element(Locator::Css("tr.trayectoRow"))
            .await?;

        let departure_hours = self.client.find_all(Locator::Css("div.salida")).await?;
        let result_hours = join_all(
            departure_hours
                .iter()
                .map(|departure_hour| departure_hour.text()),
        )
        .await;

        let hours = result_hours
            .into_iter()
            .map(|hour| hour.unwrap())
            .collect::<Vec<_>>();
        print!("departure_hours: {:?}", hours);

        Ok(())
    }

    // pub async fn search_dates(&mut self, search_filters: &SearchFilters) -> Result<(), Err> {

    // }

    pub async fn close(self) {
        self.client.close().await.expect("failed to close browser");
    }

    async fn send_keys_by_locator(
        &mut self,
        locator: Locator<'_>,
        text: &str,
    ) -> Result<(), Box<dyn Error>> {
        let origin_element = self.client.wait().for_element(locator).await?;
        origin_element.click().await?;
        origin_element.send_keys(text).await?;
        Ok(())
    }

    async fn click_element_by_locator(
        &mut self,
        locator: Locator<'_>,
    ) -> Result<(), Box<dyn Error>> {
        self.client
            .wait()
            .for_element(locator)
            .await?
            .click()
            .await?;
        Ok(())
    }

    async fn press_keys(&mut self, keys: Vec<Key>) -> Result<(), Box<dyn Error>> {
        let mut key_actions = KeyActions::new("keys".to_string());
        for key in keys {
            key_actions = key_actions.then(KeyAction::Down { value: key.into() });
        }
        self.client.perform_actions(key_actions).await?;
        Ok(())
    }
}

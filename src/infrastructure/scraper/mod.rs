pub mod constants;
pub mod utils;

use crate::Result;
use constants::*;
use fantoccini::{elements::Element, key::Key, Client, ClientBuilder, Locator};

pub struct RenfeScraper {
    client: Client,
}

impl RenfeScraper {
    pub async fn new() -> Result<Self> {
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

    pub async fn close(self) {
        self.client.close().await.expect("failed to close browser");
    }

    pub async fn buy_tickets(&mut self, search_filters: &SearchFilter<'_>) -> Result<()> {
        self.client.goto(RENFE_URL).await?;
        self.search_stations(search_filters).await?;
        self.search_departure_date(search_filters).await?;
        self.select_trains(search_filters).await
    }

    async fn search_stations(&mut self, search_filters: &SearchFilter<'_>) -> Result<()> {
        // Type the origin station
        utils::send_keys_by_locator(
            &mut self.client,
            Locator::Css("input#origin"),
            search_filters.get_origin(),
        )
        .await?;
        // Select the first option and press enter
        utils::press_keys(&mut self.client, vec![Key::Down, Key::Enter]).await?;
        // Type the destination station
        utils::send_keys_by_locator(
            &mut self.client,
            Locator::Css("input#destination"),
            search_filters.get_destination(),
        )
        .await?;
        // Select the first option and press enter
        utils::press_keys(&mut self.client, vec![Key::Down, Key::Enter]).await?;

        // Click on the "sólo ida" or "ida y vuelta" dropdown
        utils::click_element_by_locator(&mut self.client, Locator::Css("button.menu-button"))
            .await?;
        // Click on the "sólo ida" button
        utils::click_element_by_locator(
            &mut self.client,
            Locator::Css("button.rf-select__list-text"),
        )
        .await?;

        // Click on the search button
        utils::click_element_by_locator(
            &mut self.client,
            Locator::Css("button[title=\"Buscar billete\"]"),
        )
        .await?;

        Ok(())
    }

    async fn search_departure_date(&mut self, search_filters: &SearchFilter<'_>) -> Result<()> {
        // Type the departure date
        utils::send_keys_by_locator(
            &mut self.client,
            Locator::Css("input#fechaSeleccionada0"),
            search_filters.get_departure_date(),
        )
        .await
    }

    async fn select_trains(&mut self, search_filters: &SearchFilter<'_>) -> Result<()> {
        // Wait for a train row to appear
        self.client
            .wait()
            .for_element(Locator::Css("tr.trayectoRow"))
            .await?;

        let available_trains = self.client.find_all(Locator::Css("tr.trayectoRow")).await?;

        let filtered_trains = self
            .filter_trains_by_departure_hour(&available_trains, search_filters)
            .await?;

        for trian in filtered_trains {
            self.buy_ticket(&trian).await?;
        }
        Ok(())
    }

    async fn buy_ticket(&mut self, train: &Element) -> Result<()> {
        // Select the train clicking on the train row button
        train.find(Locator::Css("button")).await?.click().await?;

        // Click in the "Seleccionar" button
        utils::click_element_by_locator(
            &mut self.client,
            Locator::Css("button#buttonBannerContinuar"),
        )
        .await?;

        Ok(())
    }

    async fn filter_trains_by_departure_hour(
        &mut self,
        all_trains: &[Element],
        search_filters: &SearchFilter<'_>,
    ) -> Result<Vec<Element>> {
        let mut filtered_trains: Vec<Element> = Vec::new();

        for train in all_trains {
            let departure_hour = train.find(Locator::Css("div.salida")).await?;
            let departure_hour_text = departure_hour.text().await?;

            println!("departure_hour_text: {}", departure_hour_text);

            let departure_hour_naive = utils::get_datetime_from_string(
                search_filters.get_departure_date(),
                &departure_hour_text.replace('.', ":"),
            )?;

            let min_departure_hour_naive = utils::get_datetime_from_string(
                search_filters.get_departure_date(),
                search_filters.get_min_departure_hour(),
            )?;

            let max_departure_hour_naive = utils::get_datetime_from_string(
                search_filters.get_departure_date(),
                search_filters.get_max_departure_hour(),
            )?;

            if departure_hour_naive >= min_departure_hour_naive
                && departure_hour_naive <= max_departure_hour_naive
            {
                filtered_trains.push(train.clone());
            }
        }

        Ok(filtered_trains)
    }
}

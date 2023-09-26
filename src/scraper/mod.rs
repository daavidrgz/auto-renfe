use fantoccini::{ClientBuilder, Locator, Client};

pub struct RenfeScraper {
	client: Client,
}

impl RenfeScraper {
	pub async fn new() -> Self {
		let geckodriver_url = "http://localhost:4444";
		let renfe_url = "https://www.renfe.com/es/es";
		let c = ClientBuilder::native().connect(geckodriver_url).await.expect("failed to connect to WebDriver");
		c.goto(renfe_url).await.expect("failed to load renfe.com");
		Self {
			client: c,
		}
	}

	pub async fn find_trains(&mut self, origin: &str, destination: &str) {
		let origin_locator = Locator::Css("input#origin");
		let destination_locator = Locator::Css("input#destination");

		let current_url = self.client.current_url().await.expect("failed to get current url");
		print!("current url: {}", current_url);

		self.client.wait().for_element(origin_locator)
			.await
			.expect("failed to find origin input")
			.send_keys(origin)
			.await
			.expect("failed to send keys to origin input");

		self.client.wait().for_element(destination_locator)
			.await
			.expect("failed to find origin input")
			.send_keys(destination)
			.await
			.expect("failed to send keys to origin input");
	}

	pub async fn close(self) {
		self.client.close().await.expect("failed to close browser");
	}
}
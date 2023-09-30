use derive_builder::Builder;

pub const RENFE_URL: &str = "https://www.renfe.com/es/es";

#[derive(Default, Builder, Debug)]
pub struct SearchFilter<'a> {
    origin: &'a str,
    destination: &'a str,
    departure_date: &'a str,
}

impl SearchFilter<'_> {
    pub fn get_origin(&self) -> &str {
        self.origin
    }

    pub fn get_destination(&self) -> &str {
        self.destination
    }

    pub fn get_departure_date(&self) -> &str {
        self.departure_date
    }
}

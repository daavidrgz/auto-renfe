use derive_builder::Builder;

pub const RENFE_LOGIN_URL: &str = "https://venta.renfe.com/vol/loginCEX.do";

#[derive(Default, Builder, Debug)]
pub struct SearchFilter<'a> {
    origin: &'a str,
    destination: &'a str,
    departure_date: &'a str,
    min_departure_hour: &'a str,
    max_departure_hour: &'a str,
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

    pub fn get_min_departure_hour(&self) -> &str {
        self.min_departure_hour
    }

    pub fn get_max_departure_hour(&self) -> &str {
        self.max_departure_hour
    }
}

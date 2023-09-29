pub const RENFE_URL: &str = "https://www.renfe.com/es/es";

#[derive(Default, Builder, Debug)]
#[builder(private, setter(into))]
pub struct SearchFilter {
	origin: i32,
	destination: String,
	departure_date: String,
}
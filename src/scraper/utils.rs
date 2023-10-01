use chrono::{NaiveDateTime, ParseError};

pub fn get_datetime_from_string(date: &str, time: &str) -> Result<NaiveDateTime, ParseError> {
    NaiveDateTime::parse_from_str(&format!("{} {}", date, time), "%d/%m/%Y %H:%M")
}

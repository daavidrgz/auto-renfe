use chrono::{NaiveDateTime, ParseError};
use fantoccini::{
    actions::{InputSource, KeyAction, KeyActions},
    key::Key,
    Client, Locator,
};

pub async fn click_element_by_locator(
    _client: &mut Client,
    _locator: Locator<'_>,
) -> crate::Result<()> {
    Ok(())
}

pub async fn press_keys(client: &mut Client, keys: Vec<Key>) -> crate::Result<()> {
    let mut key_actions = KeyActions::new("keys".to_string());
    for key in keys {
        key_actions = key_actions.then(KeyAction::Down { value: key.into() });
    }
    client.perform_actions(key_actions).await?;
    Ok(())
}

pub async fn send_keys_by_locator(
    client: &mut Client,
    locator: Locator<'_>,
    text: &str,
) -> crate::Result<()> {
    let origin_element = client.wait().for_element(locator).await?;
    origin_element.click().await?;
    origin_element.send_keys(text).await?;
    Ok(())
}

pub fn get_datetime_from_string(date: &str, time: &str) -> Result<NaiveDateTime, ParseError> {
    NaiveDateTime::parse_from_str(&format!("{} {}", date, time), "%d/%m/%Y %H:%M")
}

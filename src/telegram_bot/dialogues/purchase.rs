use std::str::FromStr;

use chrono::{NaiveDate, NaiveTime};
use dptree::{case, entry};
use teloxide::{prelude::*, types::Message, Bot};

use crate::{
    entities::station::Station,
    telegram_bot::{MyDialogue, MyHandler, MyHandlerResult},
};

#[derive(Clone, Debug)]
pub enum PurchaseDialogueState {
    ReceiveStation,
    ReceiveDate(Station),
    ReceiveEarliestDepartureHour(Station, NaiveDate),
    ReceiveLatestDepartureHour(Station, NaiveDate, NaiveTime),
}

impl PurchaseDialogueState {
    pub fn schema() -> MyHandler {
        entry()
            .branch(case![Self::ReceiveStation].endpoint(Self::receive_station))
            .branch(case![Self::ReceiveDate(station)].endpoint(Self::receive_date))
            .branch(
                case![Self::ReceiveEarliestDepartureHour(station, date)]
                    .endpoint(Self::receive_earliest_departure_hour),
            )
            .branch(
                case![Self::ReceiveLatestDepartureHour(
                    station,
                    date,
                    earliest_time
                )]
                .endpoint(Self::receive_latest_departure_hour),
            )
    }

    pub async fn start(bot: Bot, dialogue: MyDialogue, msg: Message) -> MyHandlerResult {
        bot.send_message(
            msg.chat.id,
            "Purchase procedure started. Type /cancel to cancel the procedure.",
        )
        .await?;
        bot.send_message(
            msg.chat.id,
            "From which station departure do you want to buy?",
        )
        .await?;
        dialogue
            .update(PurchaseDialogueState::ReceiveStation)
            .await?;
        Ok(())
    }

    async fn receive_station(bot: Bot, dialogue: MyDialogue, msg: Message) -> MyHandlerResult {
        let chat_id = msg.chat.id;
        let text = match msg.text() {
            Some(text) => text.to_string(),
            None => {
                bot.send_message(chat_id, "Please, send the name of the station")
                    .await?;
                return Ok(());
            }
        };
        let station = match Station::from_str(&text) {
            Ok(station) => station,
            Err(None) => {
                bot.send_message(chat_id, "Please, send a valid station name")
                    .await?;
                return Ok(());
            }
            Err(Some(suggestion)) => {
                bot.send_message(
                    chat_id,
                    format!(
                        "Please, send a valid station name\n\
                        Did you mean {suggestion}?"
                    ),
                )
                .await?;
                return Ok(());
            }
        };

        let station_name = station.name();
        bot.send_message(
            chat_id,
            format!(
                "You want to buy from station {station_name}\n\
                Please, send the date of departure in the format dd/mm/yyyy",
            ),
        )
        .await?;
        dialogue
            .update(PurchaseDialogueState::ReceiveDate(station))
            .await?;

        Ok(())
    }
    async fn receive_date(
        bot: Bot,
        dialogue: MyDialogue,
        station: Station,
        msg: Message,
    ) -> MyHandlerResult {
        let text = match msg.text() {
            Some(date) => date.to_string(),
            None => {
                bot.send_message(msg.chat.id, "Please, send the date of departure")
                    .await?;
                return Ok(());
            }
        };

        let date = match NaiveDate::parse_from_str(&text, "%d/%m/%Y") {
            Ok(date) => date,
            Err(_) => {
                bot.send_message(
                    msg.chat.id,
                    "Invalid date format. Please, send the date of departure in the format dd/mm/yyyy",
                )
                .await?;
                return Ok(());
            }
        };

        let station_name = station.name();
        let date_format = date.format("%d/%m/%Y");
        bot.send_message(
            msg.chat.id,
            format!(
                "You want to buy from station {station_name} on {date_format}\n\
                Please, send the earliest time of departure in the format hh:mm 24h",
            ),
        )
        .await?;

        dialogue
            .update(PurchaseDialogueState::ReceiveEarliestDepartureHour(
                station, date,
            ))
            .await?;

        Ok(())
    }
    async fn receive_earliest_departure_hour(
        bot: Bot,
        dialogue: MyDialogue,
        (station, date): (Station, NaiveDate),
        msg: Message,
    ) -> MyHandlerResult {
        let text = match msg.text() {
            Some(date) => date.to_string(),
            None => {
                bot.send_message(msg.chat.id, "Please, send the earliest time of departure")
                    .await?;
                return Ok(());
            }
        };

        let earliest_time = match NaiveTime::parse_from_str(&text, "%H:%M") {
            Ok(time) => time,
            Err(_) => {
                bot.send_message(
                    msg.chat.id,
                    "Invalid time format. Please, send the earliest time \
                    of departure in the format hh:mm 24h",
                )
                .await?;
                return Ok(());
            }
        };

        let station_name = station.name();
        let date_format = date.format("%d/%m/%Y");
        let earliest_time_format = earliest_time.format("%H:%M");
        bot.send_message(
            msg.chat.id,
            format!(
                "You want to buy from station {station_name} on {date_format} no earlier than {earliest_time_format}\n\
                Please, send the latest time of departure in the format hh:mm 24h",
            ),
        )
        .await?;

        dialogue
            .update(PurchaseDialogueState::ReceiveLatestDepartureHour(
                station,
                date,
                earliest_time,
            ))
            .await?;

        Ok(())
    }
    async fn receive_latest_departure_hour(
        bot: Bot,
        dialogue: MyDialogue,
        (station, date, earliest_time): (Station, NaiveDate, NaiveTime),
        msg: Message,
    ) -> MyHandlerResult {
        let text = match msg.text() {
            Some(date) => date.to_string(),
            None => {
                bot.send_message(msg.chat.id, "Please, send the latest time of departure")
                    .await?;
                return Ok(());
            }
        };

        let latest_time = match NaiveTime::parse_from_str(&text, "%H:%M") {
            Ok(time) => time,
            Err(_) => {
                bot.send_message(
                    msg.chat.id,
                    "Invalid time format. Please, send the latest time of departure in the format hh:mm 24h",
                )
                .await?;
                return Ok(());
            }
        };

        let station_name = station.name();
        let date_format = date.format("%d/%m/%Y");
        let earliest_time_format = earliest_time.format("%H:%M");
        let latest_time_format = latest_time.format("%H:%M");
        bot.send_message(
            msg.chat.id,
            format!(
                "You want to buy from station {station_name} on {date_format} no earlier than \
                {earliest_time_format} and no later than {latest_time_format}"
            ),
        )
        .await?;

        dialogue.exit().await?;

        Ok(())
    }
}

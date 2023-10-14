use edit_distance::edit_distance;
use once_cell::sync::Lazy;
use std::{collections::HashSet, str::FromStr};

#[derive(Clone, Debug)]
pub struct Station {
    name: String,
}

impl Station {
    pub fn name(&self) -> &str {
        &self.name
    }
}

const SIMILARITY_THRESHOLD: f64 = 0.5;
static STATIONS: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    let stations = include_str!("valid_stations.txt")
        .lines()
        .map(|station| station.trim())
        .collect::<HashSet<_>>();
    stations
});
impl FromStr for Station {
    type Err = Option<&'static str>;

    fn from_str(name: &str) -> Result<Self, Self::Err> {
        let name_uppercase = name.to_uppercase();
        if STATIONS.contains(name_uppercase.as_str()) {
            return Ok(Self {
                name: name_uppercase,
            });
        }
        let closest_station = STATIONS
            .iter()
            .map(|station| {
                (
                    *station,
                    edit_distance(&station.to_lowercase(), &name.to_lowercase()),
                )
            })
            .min_by_key(|(_, distance)| *distance);

        let Some((closest_station,distance)) = closest_station else {
            return Err(None);
        };

        if distance == 0 {
            return Ok(Self {
                name: closest_station.to_string(),
            });
        }

        let similarity = 1.0 - (distance as f64 / closest_station.chars().count() as f64);
        if similarity > SIMILARITY_THRESHOLD {
            Err(Some(closest_station))
        } else {
            Err(None)
        }
    }
}

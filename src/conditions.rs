use serde::Serialize;

use crate::{config::Config, weather::CurrentConditions};

#[derive(Debug, Serialize)]
struct Output {
    temp: i32,
    icon: String,
}

impl From<CurrentConditions> for Output {
    fn from(conditions: CurrentConditions) -> Self {
        let temp = match Config::load() {
            Ok(config) => match config.unit.as_char() {
                'c' => conditions.temp_c,
                _ => conditions.temp_f,
            },
            Err(_) => conditions.temp_f,
        };

        Self {
            temp: temp as i32,
            icon: conditions.icon.unwrap_or_default(),
        }
    }
}

pub struct Conditions {
    config: Config,
}

impl Conditions {
    pub fn new(config: Config) -> eyre::Result<Self> {
        Ok(Self { config })
    }

    /// Fetch the current weather conditions given supplied configuration.
    ///
    /// - use configured location or infer location via IP
    /// - retrieve wather conditions from weatherapi.com for location
    /// - set icon based on conditions and time of day
    /// - compose output structure
    /// - convert output to JSON and return
    pub fn fetch(&self) -> eyre::Result<String> {
        let provider = match &self.config.weatherapi_token {
            Some(token) => {
                crate::weather::Provider::WeatherAPI(token.to_string())
            }
            None => crate::weather::Provider::OpenMeteo,
        };

        let location = self.config.get_location()?;

        let mut conditions = crate::weather::CurrentConditions::get(
            provider.clone(),
            self.config.unit,
            &location.latitude,
            &location.longitude,
        )?;

        conditions
            .set_icon(conditions.time_of_day.icon(provider, conditions.code));

        let output = Output::from(conditions);

        Ok(ureq::serde_json::to_string(&output)?)
    }
}

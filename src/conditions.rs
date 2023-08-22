use serde::Serialize;

use crate::{
    config::Config, icons, location, location::Location,
    weather::CurrentConditions,
};

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
    location: Location,
}

impl Conditions {
    pub fn new(config: Config) -> eyre::Result<Self> {
        let location = match config.get_location().unwrap_or_default() {
            Some(location) => location,
            None => {
                eprintln!(
                    "location not set, trying to infer via: {}",
                    location::from_ip::URL,
                );

                let inferred = location::get(None)?;

                eprintln!("inferred location: {}", inferred.loc);

                inferred
            }
        };

        Ok(Self { config, location })
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

        let mut conditions = crate::weather::CurrentConditions::get(
            provider.clone(),
            self.config.unit,
            &self.location.latitude,
            &self.location.longitude,
        )?;

        let time_of_day = icons::TimeOfDay::from(conditions.is_day);

        conditions.set_icon(time_of_day.icon(provider, conditions.code));

        let output = Output::from(conditions);

        Ok(ureq::serde_json::to_string(&output)?)
    }
}

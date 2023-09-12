use std::fmt;

use crate::{api::Fetchable, config::Config, location::Location};

pub(crate) mod open_meteo;
pub(crate) mod weather_api;

#[derive(Debug)]
pub struct CurrentConditions {
    pub temp_c: f32,
    pub temp_f: f32,
    pub icon: String,
}

pub trait WeatherProvider {
    fn current(&self) -> eyre::Result<CurrentConditions>;
    fn query_pairs(&self) -> Vec<(&str, &str)>;
}

#[derive(Clone, Debug)]
pub enum Provider {
    WeatherAPI,
    OpenMeteo,
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Provider::WeatherAPI => "WeatherAPI",
            Provider::OpenMeteo => "OpenMeteo",
        };
        write!(f, "{}", name)
    }
}

impl CurrentConditions {
    pub fn get(
        config: &Config,
        location: &Location,
    ) -> eyre::Result<CurrentConditions> {
        let providers = vec![Provider::WeatherAPI, Provider::OpenMeteo];

        for provider in &providers {
            let result = match provider {
                Provider::WeatherAPI => {
                    weather_api::Client::new(config, location).fetch()
                }
                Provider::OpenMeteo => {
                    open_meteo::Client::new(config, location).fetch()
                }
            };

            match result {
                Ok(conditions) => return Ok(conditions),
                Err(_) => {
                    eprintln!("error fetching weather from: {}", provider)
                }
            }
        }

        Err(eyre::eyre!("no weather providers succeeded"))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_converts_provider_to_string() {
        assert_eq!(Provider::WeatherAPI.to_string(), "WeatherAPI");
        assert_eq!(Provider::OpenMeteo.to_string(), "OpenMeteo");
    }
}

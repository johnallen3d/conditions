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

pub trait Provider {
    fn current(&self) -> eyre::Result<CurrentConditions>;
    fn query_pairs(&self) -> Vec<(&str, &str)>;
}

#[derive(Clone, Debug)]
pub enum Source {
    WeatherAPI,
    OpenMeteo,
}

impl fmt::Display for Source {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Source::WeatherAPI => "WeatherAPI",
            Source::OpenMeteo => "OpenMeteo",
        };
        write!(f, "{name}")
    }
}

impl CurrentConditions {
    pub fn get(
        config: &Config,
        location: &Location,
    ) -> eyre::Result<CurrentConditions> {
        let sources = vec![Source::WeatherAPI, Source::OpenMeteo];

        for source in &sources {
            let result = match source {
                Source::WeatherAPI => {
                    weather_api::Client::new(config, location).fetch()
                }
                Source::OpenMeteo => {
                    open_meteo::Client::new(config, location).fetch()
                }
            };

            match result {
                Ok(conditions) => return Ok(conditions),
                Err(err) => eprintln!("{err}: {source}"),
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
        assert_eq!(Source::WeatherAPI.to_string(), "WeatherAPI");
        assert_eq!(Source::OpenMeteo.to_string(), "OpenMeteo");
    }
}

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
            icon: conditions.icon,
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
    /// - retrieve wather conditions from weather provider(s) for location
    /// - compose output structure
    /// - convert output to JSON and return
    pub fn fetch(&self) -> eyre::Result<String> {
        let location = self.config.get_location()?;
        let conditions = CurrentConditions::get(&self.config, &location)?;
        let output = Output::from(conditions);

        Ok(ureq::serde_json::to_string(&output)?)
    }
}

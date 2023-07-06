use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::args::Unit;

#[derive(Error, Debug)]
pub enum ParseConfigError {
    #[error("failure to load configuration")]
    Loading(#[from] confy::ConfyError),
    #[error("configuration for {0} not found")]
    Missing(String),
}

pub const APP_NAME: &str = "conditions";
pub const CONFIG_NAME: &str = "config";

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct Config {
    pub location: Option<String>,
    pub unit: Unit,
    pub weatherapi_token: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self, ParseConfigError> {
        confy::load(APP_NAME, CONFIG_NAME).map_err(ParseConfigError::Loading)
    }

    pub fn path() -> Result<String, ParseConfigError> {
        let path = confy::get_configuration_file_path(APP_NAME, CONFIG_NAME)
            .map_err(ParseConfigError::Loading)?;

        Ok(path.display().to_string())
    }

    pub fn view() -> Result<String, ParseConfigError> {
        Ok(format!("{}", Self::load()?))
    }

    pub fn get_location(&self) -> Result<String, ParseConfigError> {
        match &self.location {
            Some(location) => Ok(location.clone()),
            None => Err(ParseConfigError::Missing("location".to_owned())),
        }
    }

    pub fn set_location(location: &str) -> Result<String, ParseConfigError> {
        let mut config = Self::load()?;

        config.location = Some(location.to_owned());
        config.store();

        Ok("location stored successfully".to_string())
    }

    pub fn set_unit(unit: Unit) -> Result<String, ParseConfigError> {
        let mut config = Self::load()?;

        config.unit = unit;
        config.store();

        Ok(format!("unit stored as: {}", unit))
    }

    pub fn get_weatherapi_token(&self) -> Result<String, ParseConfigError> {
        match &self.weatherapi_token {
            Some(token) => Ok(token.clone()),
            None => {
                Err(ParseConfigError::Missing("weatherapi token".to_owned()))
            }
        }
    }

    pub fn set_weatherapi_token(
        token: &str,
    ) -> Result<String, ParseConfigError> {
        let mut config = Self::load()?;

        config.weatherapi_token = Some(token.to_owned());
        config.store();

        Ok("weatherapi.com token stored successfully".to_owned())
    }

    pub fn store(&self) {
        confy::store(APP_NAME, CONFIG_NAME, self).expect("error saving config");
    }
}

impl fmt::Display for Config {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "Stored Configuration\n  Location: {}\n  Unit: {}\n  Token: {}",
            self.location.clone().unwrap_or_default(),
            self.unit,
            self.weatherapi_token.clone().unwrap_or_default()
        )
    }
}

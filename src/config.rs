use std::fmt;

use eyre::WrapErr;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::args::Unit;
use crate::location::{self, Location};

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
    #[serde(flatten)]
    pub location: Option<Location>,
    pub unit: Unit,
    pub weatherapi_token: Option<String>,
}

impl Config {
    pub fn load() -> eyre::Result<Self> {
        confy::load(APP_NAME, CONFIG_NAME)
            .map_err(ParseConfigError::Loading)
            .wrap_err("error loading config")
    }

    pub fn path() -> eyre::Result<String> {
        let path = confy::get_configuration_file_path(APP_NAME, CONFIG_NAME)
            .map_err(ParseConfigError::Loading)?;

        Ok(path.display().to_string())
    }

    pub fn view() -> eyre::Result<String> {
        Ok(format!("{}", Self::load()?))
    }

    pub fn get_location(&self) -> eyre::Result<Location> {
        match &self.location {
            Some(location) => Ok(location.clone()),
            None => {
                eprintln!(
                    "location not set, trying to infer via: {}",
                    location::from_ip::URL,
                );

                let inferred = location::get(None)?;

                eprintln!("inferred location: {}", inferred.loc);

                Ok(inferred)
            }
        }
    }

    pub fn set_location(region: &str) -> eyre::Result<String> {
        let mut config = Self::load()?;
        let location = location::get(Some(region))?;

        config.location = Some(location);
        config.store()?;

        Ok("location stored successfully".to_string())
    }

    pub fn set_unit(unit: Unit) -> eyre::Result<String> {
        let mut config = Self::load()?;

        config.unit = unit;
        config.store()?;

        Ok(format!("unit stored as: {}", unit))
    }

    pub fn get_weatherapi_token(&self) -> eyre::Result<String> {
        match &self.weatherapi_token {
            Some(token) => Ok(token.clone()),
            None => {
                Err(ParseConfigError::Missing("weatherapi token".to_owned()))
                    .wrap_err("error getting api token")
            }
        }
    }

    pub fn set_weatherapi_token(token: &str) -> eyre::Result<String> {
        let mut config = Self::load()?;

        config.weatherapi_token = Some(token.to_owned());
        config.store()?;

        Ok("weatherapi.com token stored successfully".to_owned())
    }

    pub fn store(&self) -> eyre::Result<()> {
        confy::store(APP_NAME, CONFIG_NAME, self)
            .wrap_err("error saving config")
    }
}

impl fmt::Display for Config {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let location = self.location.clone().unwrap_or_default();

        write!(
            fmt,
            "Stored Configuration\n  Coordinates: {}\n  Postal Code: {}\n  Unit: {}\n  Weather API Token: {}",
            location.loc.clone(),
            location.postal_code.clone(),
            self.unit,
            self.weatherapi_token.clone().unwrap_or_default()
        )
    }
}

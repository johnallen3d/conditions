use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::args::Unit;

#[derive(Error, Debug)]
pub enum ParseConfigError {
    #[error("failure to load configuration")]
    Loading(#[from] confy::ConfyError),
}

pub const APP_NAME: &str = "conditions";
pub const CONFIG_NAME: &str = "config";

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct Config {
    pub location: String,
    pub unit: Unit,
    pub weatherapi_token: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self, ParseConfigError> {
        confy::load(APP_NAME, CONFIG_NAME).map_err(ParseConfigError::Loading)
    }

    pub fn path() {
        let path = confy::get_configuration_file_path(APP_NAME, CONFIG_NAME)
            .expect("error retrieving config path");

        println!("{}", path.display());
    }

    pub fn view() -> Result<(), ParseConfigError> {
        println!("{}", Self::load()?);

        Ok(())
    }

    pub fn set_location(location: &str) -> Result<(), ParseConfigError> {
        let mut config = Self::load()?;

        config.location = location.to_owned();
        config.store();

        print!("location stored successfully");

        Ok(())
    }

    pub fn set_unit(unit: Unit) -> Result<(), ParseConfigError> {
        let mut config = Self::load()?;

        config.unit = unit;
        config.store();

        print!("unit stored as: {}", unit);

        Ok(())
    }

    pub fn set_weatherapi_token(token: &str) -> Result<(), ParseConfigError> {
        let mut config = Self::load()?;

        config.weatherapi_token = Some(token.to_owned());
        config.store();

        print!("weatherapi.com token stored successfully");

        Ok(())
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
            self.location,
            self.unit,
            self.weatherapi_token.clone().unwrap_or_default()
        )
    }
}

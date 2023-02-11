use std::fmt;

use serde::{Deserialize, Serialize};

use crate::args::Unit;

pub const APP_NAME: &str = "conditions";
pub const CONFIG_NAME: &str = "config";

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(default)]
pub struct Config {
    pub location: String,
    pub unit: char,
    pub weatherapi_token: String,
}

impl Config {
    pub fn load() -> Self {
        confy::load(APP_NAME, CONFIG_NAME).expect("error loading config")
    }

    pub fn path() {
        let path = confy::get_configuration_file_path(APP_NAME, CONFIG_NAME)
            .expect("error retrieving config path");

        println!("{}", path.display());
    }

    pub fn view() {
        println!("{}", Self::load());
    }

    pub fn set_location(location: &str) {
        let mut config = Self::load();

        config.location = location.to_owned();
        config.store();

        print!("location stored successfully");
    }

    pub fn set_unit(unit: &Unit) {
        let mut config = Self::load();

        config.unit = unit.as_char();

        config.store();

        print!("unit stored successfully");
    }

    pub fn set_weatherapi_token(token: &str) {
        let mut config = Self::load();

        config.weatherapi_token = token.to_owned();
        config.store();

        print!("weatherapi.com token stored successfully");
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
            self.location, self.unit, self.weatherapi_token
        )
    }
}

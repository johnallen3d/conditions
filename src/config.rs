use serde::{Deserialize, Serialize};

pub const APP_NAME: &str = "conditions";
pub const CONFIG_NAME: &str = "config";

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Config {
    pub location: String,
    pub unit: char,
    pub weatherapi_token: String,
}

impl Config {
    pub fn load() -> Self {
        confy::load(APP_NAME, CONFIG_NAME).unwrap()
    }

    pub fn set_location(location: &str) {
        let mut config = Self::load();

        config.location = location.to_owned();
        config.store();

        print!("location stored successfully");
    }

    pub fn set_unit(unit: char) {
        let mut config = Self::load();

        config.unit = unit;
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
        confy::store(APP_NAME, CONFIG_NAME, self).unwrap();
    }
}

use std::fmt;

use eyre::WrapErr;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{cache::Cache, location, Unit};

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
    pub location: Option<location::Location>,
    pub unit: Unit,
    pub weatherapi_token: Option<String>,
}

impl Config {
    /// Loads the configuration for the application.
    ///
    /// This function uses the `confy` crate to load the configuration for the application. It returns a `Result` with the loaded configuration on success, or an error if there was a problem loading the configuration.
    ///
    /// # Errors
    ///
    /// This function can return an error of type `eyre::Report` if there was an issue loading the configuration. The possible error scenarios include:
    ///
    /// - If there was an error parsing the configuration file.
    /// - If there was an error reading the configuration file.
    /// - If the configuration file does not exist.
    pub fn load() -> eyre::Result<Self> {
        confy::load(APP_NAME, CONFIG_NAME)
            .map_err(ParseConfigError::Loading)
            .wrap_err("error loading config")
    }

    fn path() -> eyre::Result<std::path::PathBuf> {
        let path = confy::get_configuration_file_path(APP_NAME, CONFIG_NAME)
            .map_err(ParseConfigError::Loading)?;

        Ok(path)
    }

    /// Returns the location of the current configuration file as a string.
    ///
    /// # Errors
    ///
    /// Returns an [`eyre::Result`](https://docs.rs/eyre/latest/eyre/type.Result.html) that may contain the following errors:
    ///
    /// - If the path to the configuration file cannot be retrieved, an [`eyre::Report`](https://docs.rs/eyre/latest/eyre/struct.Report.html) is returned.
    pub fn location() -> eyre::Result<String> {
        Ok(Config::path()?.display().to_string())
    }

    /// Returns the path to the cache database file.
    ///
    /// This function retrieves the path to the cache database file by first obtaining the path to the configuration file using the `Config::path()` function. It then appends the filename "cache.db" to the obtained path.
    ///
    /// # Errors
    ///
    /// This function can return an error if:
    ///
    /// - The path to the configuration file cannot be determined. This can occur if the `Config::path()` function returns an error or if the parent directory of the configuration file path cannot be obtained.
    ///
    /// # Returns
    ///
    /// - `Ok(String)`: The path to the cache database file as a string.
    pub fn cache_path() -> eyre::Result<String> {
        let mut path = Config::path()?
            .parent()
            .ok_or(eyre::eyre!("error determinig config path"))?
            .to_path_buf();

        path.push("cache.db");

        Ok(path.display().to_string())
    }

    /// Retrieves and returns a string representation of the current view.
    ///
    /// # Errors
    ///
    /// Returns an `eyre::Result` that may contain an error if there was a problem loading the view.
    ///
    /// # Returns
    ///
    /// Returns a `String` that represents the current view.
    pub fn view() -> eyre::Result<String> {
        Ok(format!("{}", Self::load()?))
    }

    /// Retrieves the location associated with the current instance.
    ///
    /// # Arguments
    ///
    /// * `cache` - A mutable reference to the cache object used for retrieving the location.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the retrieved `Location` if successful, or an `eyre::Error` if an error occurs during the retrieval process.
    ///
    /// # Errors
    ///
    /// This function can return an `eyre::Error` in the following scenarios:
    ///
    /// * If the location is not already set and the retrieval process fails.
    pub async fn get_location(
        &mut self,
        cache: &mut Cache,
    ) -> eyre::Result<location::Location> {
        match &self.location {
            Some(location) => Ok(location.clone()),
            None => {
                let inferred = location::get(cache, None).await?;

                Ok(inferred)
            }
        }
    }

    /// Sets the location of the user.
    ///
    /// This function is used to set the location of the user. It takes in the latitude and longitude coordinates as parameters and updates the user's location accordingly.
    ///
    /// # Arguments
    ///
    /// * `latitude` - A f64 representing the latitude coordinate of the user's location.
    /// * `longitude` - A f64 representing the longitude coordinate of the user's location.
    ///
    /// # Returns
    ///
    /// This function does not return anything.
    ///
    /// # Errors
    ///
    /// This function does not raise any errors.
    pub async fn set_location(
        &mut self,
        cache: &mut Cache,
        region: &str,
    ) -> eyre::Result<String> {
        let mut config = Self::load()?;
        let location = location::get(cache, Some(region)).await?;

        config.location = Some(location);
        config.store()?;

        Ok("location stored successfully".to_string())
    }

    /// Unsets the location in the configuration file.
    ///
    /// This function unsets the location in the configuration file by setting it to `None`. It then saves the updated configuration file.
    ///
    /// # Errors
    ///
    /// This function can return an error if:
    ///
    /// - The configuration file fails to load.
    /// - The configuration file fails to save.
    ///
    /// # Returns
    ///
    /// This function returns a `Result` indicating whether the operation was successful or not. If successful, it returns a `String` with the message "location unset successfully".
    pub fn unset_location() -> eyre::Result<String> {
        let mut config = Self::load()?;

        config.location = None;
        config.store()?;

        Ok("location unset successfully".to_string())
    }

    /// Sets the unit for the configuration and stores it.
    ///
    /// # Arguments
    ///
    /// * `unit` - The unit to set for the configuration.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a `String` if successful. The `String` will be a message indicating the unit that was stored.
    ///
    /// # Errors
    ///
    /// Returns an `eyre::Result` if there was an error loading or storing the configuration.
    pub fn set_unit(unit: Unit) -> eyre::Result<String> {
        let mut config = Self::load()?;

        config.unit = unit;
        config.store()?;

        Ok(format!("unit stored as: {unit}"))
    }

    /// Retrieves the `WeatherAPI` token.
    ///
    /// This function returns the `WeatherAPI` token as a `Result<String, eyre::Report>`. If the token is present, it is returned as `Ok(token)`. If the token is missing, an error is returned as `Err(ParseConfigError::Missing("weatherapi key".to_owned())).wrap_err("error getting api key")`.
    ///
    /// # Returns
    ///
    /// - `Ok(token)`: The `WeatherAPI` token as a `String`.
    /// - `Err(error)`: An error of type `eyre::Report` indicating the reason for failure.
    ///
    /// # Errors
    ///
    /// Returns an `eyre::Result` if the weather api key is missing.
    pub fn get_weatherapi_token(&self) -> eyre::Result<String> {
        match &self.weatherapi_token {
            Some(key) => Ok(key.clone()),
            None => Err(ParseConfigError::Missing("weatherapi key".to_owned()))
                .wrap_err("error getting api key"),
        }
    }

    /// Sets the `WeatherAPI` token in the configuration file.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that represents the `WeatherAPI` token to be set.
    ///
    /// # Returns
    ///
    /// Returns a Result containing a String if the token is set successfully, or an error if any operation fails.
    ///
    /// # Errors
    ///
    /// This function can return an error if any of the following operations fail:
    ///
    /// * Loading the configuration file.
    /// * Storing the updated configuration file.
    pub fn set_weatherapi_token(key: &str) -> eyre::Result<String> {
        let mut config = Self::load()?;

        config.weatherapi_token = Some(key.to_owned());
        config.store()?;

        Ok("weatherapi.com key stored successfully".to_owned())
    }

    /// Unsets the weatherapi token in the configuration file.
    ///
    /// # Errors
    ///
    /// Returns an error if there is an issue loading or storing the configuration file.
    ///
    /// # Returns
    ///
    /// Returns a `Result` indicating whether the weatherapi token was unset successfully or not.
    ///
    /// If successful, returns a `String` with the message "weatherapi.com key unset successfully".
    pub fn unset_weatherapi_token() -> eyre::Result<String> {
        let mut config = Self::load()?;

        config.weatherapi_token = None;
        config.store()?;

        Ok("weatherapi.com key unset successfully".to_owned())
    }

    /// Stores the current configuration in the specified file.
    ///
    /// # Errors
    ///
    /// Returns an `eyre::Result` that wraps an error if there was a problem saving the configuration.
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
            "Stored Configuration\n  Coordinates: {}\n  Postal Code: {}\n  Unit: {}\n  Weather API Key: {}",
            location.loc.clone(),
            location.postal_code.clone(),
            self.unit,
            self.weatherapi_token.clone().unwrap_or_default()
        )
    }
}

#![deny(clippy::pedantic)]

use clap::Parser;

use args::{
    Command, Conditions, ConfigSubcommand, LocationSubcommand, UnitSubcommand,
    WeatherApiKeySubcommand,
};
use config::Config;

pub(crate) mod api;
pub mod args;
mod conditions;
mod config;
pub mod icons;
mod location;
mod weather;

/// Entry point for running the application logic based on parsed CLI arguments.
///
/// # Returns
///
/// - `Ok(String)`: A success message or relevant output for the executed command
/// - `Err(eyre::Error)`: An error if any step in the execution fails.
///
/// # Errors
///
/// This function will return an error in the following situations:
///
/// - Failure to read or write to the config file.
/// - Failure to fetch weather conditions.
/// - Failure to manage location, weather API key, or unit settings.
pub fn run() -> eyre::Result<String> {
    let args = Conditions::parse();

    let result = match &args.command {
        Command::Config(cmd) => match &cmd.command {
            ConfigSubcommand::Path => Config::path()?,
            ConfigSubcommand::View => Config::view()?,
        },
        Command::Current => {
            conditions::Conditions::new(Config::load()?).fetch()?
        }
        Command::Location(cmd) => match &cmd.command {
            LocationSubcommand::Set(input) => {
                Config::set_location(&input.region)?
            }
            LocationSubcommand::View => {
                Config::load()?.get_location()?.to_string()
            }
            LocationSubcommand::Unset => Config::unset_location()?,
        },
        Command::WeatherApiKey(cmd) => match &cmd.command {
            WeatherApiKeySubcommand::Set(input) => {
                Config::set_weatherapi_token(&input.key)?
            }
            WeatherApiKeySubcommand::View => {
                let token = Config::load()?.get_weatherapi_token()?;

                format!("token stored as: {token}")
            }
            WeatherApiKeySubcommand::Unset => Config::unset_weatherapi_token()?,
        },
        Command::Unit(cmd) => match &cmd.command {
            UnitSubcommand::Set(unit) => Config::set_unit(unit.unit)?,
            UnitSubcommand::View => {
                format!("unit stored as: {}", Config::load()?.unit)
            }
        },
    };

    Ok(result)
}

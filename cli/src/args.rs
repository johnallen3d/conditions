use std::fmt;

use clap::{Args, Parser, Subcommand};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct Conditions {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// View configuration information
    Config(ConfigCommand),
    /// Get the current weather conditions (optional provide location)
    Current { region: Option<String> },
    /// Location conditions apply to
    Location(LocationCommand),
    /// weatherapi.com api-key
    WeatherApiKey(WeatherApiKeyCommand),
    /// Weather unit, celsius or fahrenheit
    Unit(UnitCommand),
}

#[derive(Debug, Args)]
pub struct ConfigCommand {
    #[clap(subcommand)]
    pub command: ConfigSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum ConfigSubcommand {
    /// Print path to configuration file
    Path,
    /// Print all stored configuration values
    View,
}

#[derive(Debug, Args)]
pub struct WeatherApiKeyCommand {
    #[clap(subcommand)]
    pub command: WeatherApiKeySubcommand,
}

#[derive(Debug, Subcommand)]
pub enum WeatherApiKeySubcommand {
    /// Store your API key
    Set(SetApiKey),
    /// Display the current key
    View,
    /// Remove the stored key
    Unset,
}

#[derive(Debug, Args)]
pub struct SetApiKey {
    /// Your weatherapi.com key
    pub key: String,
}

#[derive(Debug, Args)]
pub struct LocationCommand {
    #[clap(subcommand)]
    pub command: LocationSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum LocationSubcommand {
    /// Store your location
    Set(SetLocation),
    /// View stored location
    View,
    /// Unset current location
    Unset,
}

#[derive(Debug, Args)]
pub struct SetLocation {
    /// Postal code and country to retrieve weather for: example - 10001,usa
    pub region: String,
}

#[derive(Debug, Args)]
pub struct UnitCommand {
    #[clap(subcommand)]
    pub command: UnitSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum UnitSubcommand {
    /// Store your unit
    Set(SetUnit),
    /// View stored unit
    View,
}

#[derive(Debug, Args)]
pub struct SetUnit {
    /// Temperature unit to return
    #[clap(value_enum)]
    pub unit: Unit,
}

#[derive(clap::ValueEnum, Clone, Copy, Debug, Default, Serialize)]
pub enum Unit {
    C,
    #[default]
    F,
}

impl Unit {
    pub fn to(self) -> conditions::Unit {
        match self {
            Unit::C => conditions::Unit::C,
            Unit::F => conditions::Unit::F,
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Unit::C => "celsius",
            Unit::F => "fahrenheit",
        };
        write!(f, "{text}")
    }
}

impl<'de> Deserialize<'de> for Unit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        match s.as_str() {
            "c" => Ok(Unit::C),
            _ => Ok(Unit::F),
        }
    }
}

use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct ConditionsArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// View configuration information
    Config(ConfigCommand),
    /// Get the current weather conditions
    Current,
    /// Location conditions apply to
    Location(LocationCommand),
    /// weatherapi.com token
    Token(TokenCommand),
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
pub struct TokenCommand {
    #[clap(subcommand)]
    pub command: TokenSubcommand,
}

#[derive(Debug, Subcommand)]
pub enum TokenSubcommand {
    /// Store your token
    Set(SetToken),
    /// View stored token
    View,
}

#[derive(Debug, Args)]
pub struct SetToken {
    /// Your weatherapi.com token
    pub token: String,
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
}

#[derive(Debug, Args)]
pub struct SetLocation {
    /// Location to retrieve weather for ("lat,long" or "city,state")
    pub location: String,
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
    #[clap(default_value = "f", value_enum)]
    pub unit: Unit,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Unit {
    C,
    F,
}

impl Unit {
    pub fn as_char(&self) -> char {
        match self {
            Unit::C => 'c',
            Unit::F => 'f',
        }
    }
}

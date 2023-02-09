use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct ConditionsArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Get the current weather conditions
    Current,
    /// Location, stored or inferred
    Location(LocationCommand),
    /// weatherapi.com token
    Token(TokenCommand),
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

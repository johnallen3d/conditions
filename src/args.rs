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

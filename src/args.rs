use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(version, about)]
pub struct ConditionsArgs {
    #[clap(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    SetToken(SetTokenCommand),
    Current,
}

#[derive(Debug, Args)]
pub struct SetTokenCommand {
    /// Your weatherapi.com token
    pub token: String,
}

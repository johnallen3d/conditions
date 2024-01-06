#![deny(clippy::pedantic)]

use clap::Parser;

mod args;

use args::{
    Command, Conditions, ConfigSubcommand, LocationSubcommand, UnitSubcommand,
    WeatherApiKeySubcommand,
};
use conditions::{cache::Cache, config::Config};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    env_logger::init();

    match run().await {
        Ok(result) => println!("{result}"),
        Err(err) => {
            // for user
            eprintln!("{err}");
            // for development
            log::error!("{:?}", err);
        }
    }
}

/// Entry point for running the application logic based on parsed CLI arguments.
///
/// # Returns
///
/// - `Ok(String)`: A success message or relevant output for the executed
///   command
/// - `Err(eyre::Error)`: An error if any step in the execution fails.
///
/// # Errors
///
/// This function will return an error in the following situations:
///
/// - Failure to read or write to the config file.
/// - Failure to fetch weather conditions.
/// - Failure to manage location, weather API key, or unit settings.
pub async fn run() -> eyre::Result<String> {
    let args = Conditions::parse();

    let result = match &args.command {
        Command::Config(cmd) => match &cmd.command {
            ConfigSubcommand::Path => Config::location()?,
            ConfigSubcommand::View => Config::view()?,
        },
        Command::Current { region } => {
            let (config, mut cache) = init().await?;

            let output = conditions::Conditions::new(config, region.clone())
                .fetch(&mut cache)
                .await?;

            serde_json::to_string(&output)?
        }
        Command::Location(cmd) => match &cmd.command {
            LocationSubcommand::Set(input) => {
                let (mut config, mut cache) = init().await?;

                config.set_location(&mut cache, &input.region).await?
            }
            LocationSubcommand::View => {
                let (mut config, mut cache) = init().await?;

                config.get_location(&mut cache).await?.to_string()
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
            UnitSubcommand::Set(unit) => Config::set_unit(unit.unit.to())?,
            UnitSubcommand::View => {
                format!("unit stored as: {}", Config::load()?.unit)
            }
        },
    };

    Ok(result)
}

async fn init() -> eyre::Result<(Config, Cache)> {
    let config = Config::load()?;
    let cache = Cache::new(Some(Config::cache_path()?)).await?;

    Ok((config, cache))
}

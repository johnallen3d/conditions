use clap::Parser;

use args::*;
use config::Config;

pub mod args;
mod conditions;
mod config;
pub mod icons;
mod location;
mod weather;

pub fn run() -> eyre::Result<String> {
    let args = ConditionsArgs::parse();

    let result = match &args.command {
        Command::Config(cmd) => match &cmd.command {
            ConfigSubcommand::Path => Config::path()?,
            ConfigSubcommand::View => Config::view()?,
        },
        Command::Current => {
            conditions::Conditions::new(Config::load()?)?.fetch()?
        }
        Command::Location(cmd) => match &cmd.command {
            LocationSubcommand::Set(input) => {
                Config::set_location(&input.region)?
            }
            LocationSubcommand::View => {
                Config::load()?.get_location()?.to_string()
            }
        },
        Command::WeatherApiKey(cmd) => match &cmd.command {
            WeatherApiKeySubcommand::Set(input) => {
                Config::set_weatherapi_token(&input.key)?
            }
            WeatherApiKeySubcommand::View => {
                let token = Config::load()?.get_weatherapi_token()?;

                format!("token stored as: {}", token)
            }
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

#[cfg(test)]
mod tests {
    use crate::config::Config;

    #[test]
    fn test_from() {
        println!("{}", Config::load().unwrap().unit);
    }
}

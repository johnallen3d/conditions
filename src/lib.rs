use std::error::Error;

use clap::Parser;
use serde::Serialize;

use args::*;
use config::Config;
use weather::Conditions;

pub mod args;
mod config;
pub mod icons;
mod location;
mod weather;

pub fn run() -> Result<(), Box<dyn Error>> {
    let args = ConditionsArgs::parse();

    match &args.command {
        Command::Config(cmd) => match &cmd.command {
            ConfigSubcommand::Path => Config::path(),
            ConfigSubcommand::View => Config::view()?,
        },
        Command::Current => current_conditions()?,
        Command::Location(cmd) => match &cmd.command {
            LocationSubcommand::Set(location) => {
                let _ = Config::set_location(&location.location);
            }
            LocationSubcommand::View => {
                Config::load()?.get_location()?;
            }
        },
        Command::Token(cmd) => match &cmd.command {
            TokenSubcommand::Set(token) => {
                Config::set_weatherapi_token(&token.token)?
            }
            TokenSubcommand::View => {
                let token = Config::load()?.get_weatherapi_token()?;

                println!("token stored as: {}", token);
            }
        },
        Command::Unit(cmd) => match &cmd.command {
            UnitSubcommand::Set(unit) => Config::set_unit(unit.unit)?,
            UnitSubcommand::View => {
                println!("unit stored as: {}", Config::load()?.unit)
            }
        },
    }

    Ok(())
}

#[derive(Debug, Serialize)]
struct Output {
    temp: i32,
    icon: String,
}

impl From<weather::Conditions> for Output {
    fn from(conditions: weather::Conditions) -> Self {
        let temp = match Config::load() {
            Ok(config) => match config.unit.as_char() {
                'c' => conditions.temp_c,
                _ => conditions.temp_f,
            },
            Err(_) => conditions.temp_f,
        };

        Self {
            temp: temp as i32,
            icon: conditions.icon.unwrap_or_default(),
        }
    }
}

fn current_conditions() -> Result<(), Box<dyn Error>> {
    let config = Config::load()?;

    let location = match config.location {
        Some(location) => location,
        None => {
            eprintln!(
                "location not set, trying to infer via: {}",
                location::LOCATION_URL,
            );

            let client = crate::location::UreqClient;
            let inferred = location::current(&client)?.to_string();

            eprintln!("inferred location: {}", inferred);

            inferred
        }
    };

    let weatherapi_token = match config.weatherapi_token {
        Some(token) => token,
        None => return Err("weatherapi token not set".into()),
    };

    let mut conditions = Conditions::current(&weatherapi_token, &location)?;
    let time_of_day = icons::TimeOfDay::from(conditions.is_day);

    conditions.set_icon(time_of_day.icon(conditions.code));

    let output = Output::from(conditions);

    println!("{}", ureq::serde_json::to_string(&output)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::config::Config;

    #[test]
    fn test_from() {
        println!("{}", Config::load().unwrap().unit);
    }
}

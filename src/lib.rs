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
mod unit;
mod weather;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
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
                println!("{}", Config::load()?.location)
            }
        },
        Command::Token(cmd) => match &cmd.command {
            TokenSubcommand::Set(token) => {
                Config::set_weatherapi_token(&token.token)?
            }
            TokenSubcommand::View => {
                println!("{}", Config::load()?.weatherapi_token)
            }
        },
        Command::Unit(cmd) => match &cmd.command {
            UnitSubcommand::Set(unit) => Config::set_unit(&unit.unit)?,
            UnitSubcommand::View => {
                println!("{}", Config::load()?.unit)
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
            Ok(config) => {
                match config.unit {
                    'c' => conditions.temp_c,
                    _ => conditions.temp_f,
                }
            }
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

    let location = if config.location.is_empty() {
        let client = crate::location::UreqClient;
        location::current(&client).unwrap_or_default().to_string()
    } else {
        config.location
    };

    let weatherapi_token = config.weatherapi_token;

    let mut conditions = Conditions::current(&weatherapi_token, &location)
        .expect("error retrieving weather conditions");

    let time_of_day = match conditions.is_day {
        true => icons::TimeOfDay::Day,
        _ => icons::TimeOfDay::Night,
    };

    conditions.set_icon(icons::icon_for(time_of_day, conditions.code));

    let output = Output::from(conditions);

    println!(
        "{}",
        ureq::serde_json::to_string(&output).expect("unexpected error")
    );

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

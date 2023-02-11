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

pub fn run() {
    let args = ConditionsArgs::parse();

    match &args.command {
        Command::Config(cmd) => match &cmd.command {
            ConfigSubcommand::Path => Config::path(),
            ConfigSubcommand::View => Config::view(),
        },
        Command::Current => current_conditions(),
        Command::Location(cmd) => match &cmd.command {
            LocationSubcommand::Set(location) => {
                Config::set_location(&location.location);
            }
            LocationSubcommand::View => println!("{}", Config::load().location),
        },
        Command::Token(cmd) => match &cmd.command {
            TokenSubcommand::Set(token) => {
                Config::set_weatherapi_token(&token.token)
            }
            TokenSubcommand::View => {
                println!("{}", Config::load().weatherapi_token)
            }
        },
        Command::Unit(cmd) => match &cmd.command {
            UnitSubcommand::Set(unit) => Config::set_unit(&unit.unit),
            UnitSubcommand::View => {
                println!("{}", Config::load().unit)
            }
        },
    }
}

#[derive(Debug, Serialize)]
struct Output {
    temp: i32,
    icon: String,
}

impl From<weather::Conditions> for Output {
    fn from(conditions: weather::Conditions) -> Self {
        let temp = if Config::load().unit == 'c' {
            conditions.temp_c
        } else {
            conditions.temp_f
        };

        Self {
            temp: temp as i32,
            icon: conditions.icon.unwrap_or_default(),
        }
    }
}

fn current_conditions() {
    let config = Config::load();

    let location = if config.location.is_empty() {
        location::current().unwrap_or_default().to_string()
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
}

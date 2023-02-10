use clap::Parser;

use args::*;
use config::Config;

mod args;
pub mod conditions;
mod config;
pub mod icons;
pub mod location;

pub fn run() {
    let args = ConditionsArgs::parse();

    match &args.command {
        Command::Current => current_conditions(),
        Command::Location(cmd) => match &cmd.command {
            LocationSubcommand::Set(location) => {
                Config::set_location(&location.location);
            }
            LocationSubcommand::View => view_location(),
        },
        Command::Token(cmd) => match &cmd.command {
            TokenSubcommand::Set(token) => {
                Config::set_weatherapi_token(&token.token)
            }
            TokenSubcommand::View => view_weatherapi_token(),
        },
    }
}

fn current_conditions() {
    let config = Config::load();

    let location = if config.location.is_empty() {
        location::current().unwrap().to_string()
    } else {
        config.location
    };

    let weatherapi_token = config.weatherapi_token;

    let mut conditions =
        conditions::current(&weatherapi_token, &location).unwrap();

    let time_of_day = match conditions.is_day {
        true => icons::TimeOfDay::Day,
        _ => icons::TimeOfDay::Night,
    };

    conditions.set_icon(icons::icon_for(time_of_day, conditions.code));

    println!("{}", ureq::serde_json::to_string(&conditions).unwrap());
}

fn view_location() {
    let config = Config::load();

    println!("{}", config.location);
}

fn view_weatherapi_token() {
    let config = Config::load();

    println!("{}", config.weatherapi_token);
}

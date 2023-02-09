use clap::Parser;
use serde::{Deserialize, Serialize};

use args::*;

mod args;
pub mod conditions;
pub mod icons;
pub mod location;

const APP_NAME: &str = "conditions";
const CONFIG_NAME: &str = "config";

#[derive(Deserialize, Serialize, Debug, Default)]
struct Config {
    weatherapi_token: String,
    location: String,
}

pub fn run() {
    let args = ConditionsArgs::parse();

    match &args.command {
        Command::Current => current_conditions(),
        Command::Location(cmd) => match &cmd.command {
            LocationSubcommand::Set(location) => {
                set_location(&location.location);
            }
            LocationSubcommand::View => view_location(),
        },
        Command::Token(cmd) => match &cmd.command {
            TokenSubcommand::Set(token) => set_weatherapi_token(&token.token),
            TokenSubcommand::View => view_weatherapi_token(),
        },
    }
}

fn load_config() -> Config {
    confy::load(APP_NAME, CONFIG_NAME).unwrap()
}

fn current_conditions() {
    let config = load_config();

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

fn set_location(location: &str) {
    let mut config = load_config();

    config.location = location.to_owned();

    confy::store(APP_NAME, CONFIG_NAME, config).unwrap();

    print!("location stored successfully");
}

fn view_location() {
    let config = load_config();

    println!("{}", config.location);
}

fn set_weatherapi_token(token: &str) {
    let mut config = load_config();

    config.weatherapi_token = token.to_owned();

    confy::store(APP_NAME, CONFIG_NAME, config).unwrap();

    print!("weatherapi.com token stored successfully");
}

fn view_weatherapi_token() {
    let config: Config = confy::load(APP_NAME, CONFIG_NAME).unwrap();

    println!("{}", config.weatherapi_token);
}

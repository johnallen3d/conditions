use clap::Parser;
use serde::{Deserialize, Serialize};

use args::{Command, ConditionsArgs};

mod args;
pub mod conditions;
pub mod icons;
pub mod location;

const APP_NAME: &str = "conditions";
const CONFIG_NAME: &str = "config";

#[derive(Deserialize, Serialize, Debug, Default)]
struct Config {
    weather_api_token: String,
}

pub fn run() {
    let args = ConditionsArgs::parse();

    match &args.command {
        Command::SetToken(cmd) => set_weatherapi_token(&cmd.token),
        Command::Current => current_conditions(),
    }
}

fn set_weatherapi_token(token: &str) {
    let config = Config {
        weather_api_token: token.to_owned(),
    };

    confy::store(APP_NAME, CONFIG_NAME, config).unwrap();
}

fn current_conditions() {
    let config: Config = confy::load(APP_NAME, CONFIG_NAME).unwrap();
    let weatherapi_token = config.weather_api_token;
    let location = location::current().unwrap();

    let mut conditions =
        conditions::current(&weatherapi_token, &location).unwrap();

    let time_of_day = match conditions.is_day {
        true => icons::TimeOfDay::Day,
        _ => icons::TimeOfDay::Night,
    };

    conditions.set_icon(icons::icon_for(time_of_day, conditions.code));

    println!("{}", ureq::serde_json::to_string(&conditions).unwrap());
}

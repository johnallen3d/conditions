use std::env;

pub mod conditions;
pub mod icons;
pub mod location;

fn main() {
    // TODO accept this as arg or read from a config file
    let weatherapi_token = env::var("WEATHERAPI_TOKEN").unwrap();

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

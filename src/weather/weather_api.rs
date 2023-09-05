use std::convert::From;

use serde::Deserialize;
use thiserror::Error;

use super::{CurrentConditions, WeatherProvider};
use crate::icons::TimeOfDay;

static URL: &str = "http://api.weatherapi.com/v1/current.jso";

pub struct Client {
    key: String,
    location: String,
}

impl Client {
    pub fn new(key: String, latitude: String, longitude: String) -> Self {
        let location = format!("{},{}", latitude, longitude);

        Self { key, location }
    }
}

impl WeatherProvider for Client {
    fn current(&self) -> eyre::Result<CurrentConditions> {
        let parsed = ureq::get(URL)
            .query_pairs(self.query_pairs())
            .call()?
            .into_json::<WeatherAPIResult>()?;

        Ok(CurrentConditions::from(parsed))
    }

    fn query_pairs(&self) -> Vec<(&str, &str)> {
        vec![("key", self.key.as_str()), ("q", self.location.as_str())]
    }
}

#[derive(Error, Debug, PartialEq)]
pub enum FetchConditionsError {
    #[error("unknown error fetching weather conditions")]
    Unknown,
}

impl std::convert::From<ureq::Error> for FetchConditionsError {
    fn from(_: ureq::Error) -> Self {
        Self::Unknown
    }
}

impl std::convert::From<std::io::Error> for FetchConditionsError {
    fn from(_: std::io::Error) -> Self {
        Self::Unknown
    }
}

/// PrasedWeather represented as JSON response
/// {
///   "current": {
///     "condition": {
///       "code": i32
///     },
///     "temp_f": f32,
///     "is_day": u8
///   }
/// }
#[derive(Debug, Deserialize)]
struct WeatherAPIResult {
    current: WeatherAPIResultCurrent,
}

#[derive(Debug, Deserialize)]
struct WeatherAPIResultCurrent {
    condition: WeatherAPIResultCondition,
    temp_c: f32,
    temp_f: f32,
    is_day: u8,
}

#[derive(Debug, Deserialize)]
struct WeatherAPIResultCondition {
    code: i32,
}

impl From<WeatherAPIResult> for CurrentConditions {
    fn from(result: WeatherAPIResult) -> Self {
        Self {
            code: result.current.condition.code,
            temp_c: result.current.temp_c,
            temp_f: result.current.temp_f,
            time_of_day: TimeOfDay::from(result.current.is_day),
            icon: None,
            provider: super::Provider::WeatherAPI("".to_string()),
        }
    }
}

use std::convert::From;

use serde::Deserialize;
use thiserror::Error;

use super::CurrentConditions;
use crate::icons::TimeOfDay;

pub struct Client {
    query: Vec<(String, String)>,
}

impl Client {
    pub fn new(key: String, latitude: String, longitude: String) -> Self {
        let location = format!("{},{}", latitude, longitude);

        Self {
            query: vec![("key".to_string(), key), ("q".to_string(), location)],
        }
    }
}

impl crate::api::Fetchable<Response, CurrentConditions> for Client {
    const URL: &'static str = "http://api.weatherapi.com/v1/current.json";

    fn query(&self) -> Option<&Vec<(String, String)>> {
        Some(&self.query)
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
pub struct Response {
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

impl From<Response> for CurrentConditions {
    fn from(result: Response) -> Self {
        let icon = TimeOfDay::from(result.current.is_day).icon(
            super::Provider::WeatherAPI("".to_string()),
            result.current.condition.code,
        );

        Self {
            temp_c: result.current.temp_c,
            temp_f: result.current.temp_f,
            icon,
        }
    }
}

use std::convert::From;

use serde::Deserialize;

static WEATHERAPI_URL: &str = "http://api.weatherapi.com/v1/current.json";

use thiserror::Error;

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

#[derive(Debug, Default)]
pub struct Conditions {
    pub code: i32,
    pub temp_c: f32,
    pub temp_f: f32,
    pub is_day: bool,
    pub icon: Option<String>,
}

impl From<WeatherAPIResult> for Conditions {
    fn from(result: WeatherAPIResult) -> Self {
        Self {
            code: result.current.condition.code,
            temp_c: result.current.temp_c,
            temp_f: result.current.temp_f,
            is_day: result.current.is_day == 1,
            icon: None,
        }
    }
}

impl Conditions {
    pub fn current(
        key: &str,
        location: &str,
    ) -> Result<Self, FetchConditionsError> {
        let query = vec![("key", key), ("q", location)];

        let parsed = ureq::get(WEATHERAPI_URL)
            .query_pairs(query)
            .call()?
            .into_json::<WeatherAPIResult>()?;

        Ok(Self::from(parsed))
    }

    pub fn set_icon(&mut self, value: String) {
        self.icon = Some(value);
    }
}

use std::convert::From;

use serde::Deserialize;

static WEATHERAPI_URL: &str = "http://api.weatherapi.com/v1/current.json";

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
    pub fn current(key: &str, location: &str) -> Result<Self, String> {
        let query = vec![("key", key), ("q", location)];

        match ureq::get(WEATHERAPI_URL)
            .query_pairs(query)
            .call()
            .unwrap()
            .into_json::<WeatherAPIResult>()
        {
            Ok(parsed) => Ok(Self::from(parsed)),
            Err(error) => Err(format!("error retrieving weather: {error}")),
        }
    }

    pub fn set_icon(&mut self, value: String) {
        self.icon = Some(value);
    }
}

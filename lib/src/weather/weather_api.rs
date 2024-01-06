use std::convert::From;

use serde::Deserialize;
use thiserror::Error;

use super::CurrentConditions;
use crate::{config::Config, icons::TimeOfDay, location::Location};

pub struct Client {
    is_valid: bool,
    query: Vec<(String, String)>,
}

impl Client {
    pub fn new(config: &Config, location: &Location) -> Self {
        let key = config.weatherapi_token.clone();
        let is_valid = key.is_some();

        Self {
            is_valid,
            query: vec![
                ("key".to_string(), key.unwrap_or_default()),
                ("q".to_string(), location.loc.clone()),
            ],
        }
    }
}

impl crate::api::Fetchable<Response, CurrentConditions> for Client {
    fn url(&self) -> &'static str {
        "http://api.weatherapi.com/v1/current.json"
    }

    fn is_valid(&self) -> bool {
        self.is_valid
    }

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

/// Response represented as JSON response
/// ```json
/// {
///   "current": {
///     "condition": {
///       "code": i32
///     },
///     "temp_f": f32,
///     "is_day": u8
///   }
/// }
/// ```
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
        let icon = TimeOfDay::from(result.current.is_day)
            .icon(&super::Source::WeatherAPI, result.current.condition.code);

        Self {
            temp_c: result.current.temp_c,
            temp_f: result.current.temp_f,
            icon,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_creates_client_with_query() {
        let config = Config {
            weatherapi_token: Some("token123".to_string()),
            ..Default::default()
        };
        let location = Location {
            loc: "loc_string".to_string(),
            ..Default::default()
        };
        let client = Client::new(&config, &location);

        assert_eq!(
            client.query,
            vec![
                ("key".to_string(), "token123".to_string()),
                ("q".to_string(), "loc_string".to_string())
            ]
        );
    }

    #[test]
    fn test_weatherapi_from() {
        let response = Response {
            current: WeatherAPIResultCurrent {
                condition: WeatherAPIResultCondition { code: 1087 },
                temp_c: 10.0,
                temp_f: 50.0,
                is_day: 0,
            },
        };
        let conditions = CurrentConditions::from(response);
        assert!((conditions.temp_c - 10.0).abs() < f32::EPSILON);
        assert!((conditions.temp_f - 50.0).abs() < f32::EPSILON);
        assert_eq!(conditions.icon, "".to_string());
    }
}

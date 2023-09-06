use serde::Deserialize;

use super::{CurrentConditions, Provider};
use crate::icons::TimeOfDay;

// {
//   "latitude": 35.159126,
//   "longitude": -80.81137,
//   "generationtime_ms": 0.1900196075439453,
//   "utc_offset_seconds": 0,
//   "timezone": "GMT",
//   "timezone_abbreviation": "GMT",
//   "elevation": 219,
//   "current_weather": {
//     "temperature": 95.9,
//     "windspeed": 15.5,
//     "winddirection": 245,
//     "weathercode": 0,
//     "is_day": 1,
//     "time": "2023-08-14T19:00"
//   }
// }
//
pub struct Client {
    query: Vec<(String, String)>,
}

impl Client {
    pub fn new(unit: crate::Unit, latitude: String, longitude: String) -> Self {
        Self {
            query: vec![
                ("current_weather".to_string(), "true".to_string()),
                ("temperature_unit".to_string(), unit.to_string()),
                ("latitude".to_string(), latitude),
                ("longitude".to_string(), longitude),
            ],
        }
    }
}

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    temperature: f32,
    weathercode: i32,
    is_day: u8,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    current_weather: CurrentWeather,
}

impl crate::api::Fetchable<Response, CurrentConditions> for Client {
    const URL: &'static str = "https://api.open-meteo.com/v1/forecast";

    fn query(&self) -> Option<&Vec<(String, String)>> {
        Some(&self.query)
    }
}

impl From<Response> for CurrentConditions {
    fn from(result: Response) -> Self {
        let icon = TimeOfDay::from(result.current_weather.is_day)
            .icon(Provider::OpenMeteo, result.current_weather.weathercode);

        Self {
            temp_c: result.current_weather.temperature,
            temp_f: result.current_weather.temperature,
            icon,
        }
    }
}

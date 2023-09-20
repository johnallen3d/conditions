use serde::Deserialize;

use super::{CurrentConditions, Source};
use crate::{icons::TimeOfDay, location::Location, Config};

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
    pub fn new(config: &Config, location: &Location) -> Self {
        let unit = config.unit.to_string();

        Self {
            query: vec![
                ("current_weather".to_string(), "true".to_string()),
                ("temperature_unit".to_string(), unit),
                ("latitude".to_string(), location.latitude.clone()),
                ("longitude".to_string(), location.longitude.clone()),
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
    fn url(&self) -> &'static str {
        "https://api.open-meteo.com/v1/forecast"
    }

    fn query(&self) -> Option<&Vec<(String, String)>> {
        Some(&self.query)
    }
}

impl From<Response> for CurrentConditions {
    fn from(result: Response) -> Self {
        let icon = TimeOfDay::from(result.current_weather.is_day)
            .icon(&Source::OpenMeteo, result.current_weather.weathercode);

        Self {
            temp_c: result.current_weather.temperature,
            temp_f: result.current_weather.temperature,
            icon,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::args::Unit;

    #[test]
    fn test_client_new() {
        let config = Config {
            unit: Unit::F,
            ..Default::default()
        };
        let location = Location::default();
        let client = Client::new(&config, &location);

        assert_eq!(client.query.len(), 4);
        assert_eq!(client.query[0].0, "current_weather");
        assert_eq!(client.query[0].1, "true");
        assert_eq!(client.query[1].0, "temperature_unit");
        assert_eq!(client.query[1].1, "fahrenheit");
        assert_eq!(client.query[2].0, "latitude");
        assert_eq!(client.query[2].1, location.latitude);
        assert_eq!(client.query[3].0, "longitude");
        assert_eq!(client.query[3].1, location.longitude);
    }

    #[test]
    fn it_converts_response_to_current_conditions() {
        let response = Response {
            current_weather: CurrentWeather {
                temperature: 10.0,
                weathercode: 85,
                is_day: 1,
            },
        };
        let conditions = CurrentConditions::from(response);

        assert!((conditions.temp_c - 10.0).abs() < f32::EPSILON);
        assert!((conditions.temp_f - 10.0).abs() < f32::EPSILON);
        assert_eq!(conditions.icon, " ");
    }
}

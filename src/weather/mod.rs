use crate::icons::TimeOfDay;

pub(crate) mod open_meteo;
pub(crate) mod weather_api;

#[derive(Debug)]
pub struct CurrentConditions {
    pub code: i32,
    pub temp_c: f32,
    pub temp_f: f32,
    pub time_of_day: TimeOfDay,
    pub icon: Option<String>,
    pub provider: Provider,
}

pub trait WeatherProvider {
    fn current(&self) -> eyre::Result<CurrentConditions>;
    fn query_pairs(&self) -> Vec<(&str, &str)>;
}

#[derive(Clone, Debug)]
pub enum Provider {
    // contains api key
    WeatherAPI(String),
    OpenMeteo,
}

impl CurrentConditions {
    pub fn get(
        providers: Vec<Provider>,
        unit: crate::Unit,
        latitude: &str,
        longitude: &str,
    ) -> eyre::Result<CurrentConditions> {
        for provider in providers {
            let client: Box<dyn WeatherProvider> = match provider {
                Provider::WeatherAPI(key) => {
                    Box::new(weather_api::Client::new(
                        key,
                        latitude.to_string(),
                        longitude.to_string(),
                    ))
                }
                Provider::OpenMeteo => Box::new(open_meteo::Client::new(
                    unit,
                    latitude.to_string(),
                    longitude.to_string(),
                )),
            };

            if let Ok(conditions) = client.current() {
                return Ok(conditions);
            }
        }

        Err(eyre::eyre!("no weather providers succeeded"))
    }

    pub fn set_icon(&mut self, value: String) {
        self.icon = Some(value);
    }
}

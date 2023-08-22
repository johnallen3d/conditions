pub(crate) mod open_meteo;
pub(crate) mod weather_api;

#[derive(Debug, Default)]
pub struct CurrentConditions {
    pub code: i32,
    pub temp_c: f32,
    pub temp_f: f32,
    pub is_day: bool,
    pub icon: Option<String>,
}

pub trait WeatherProvider {
    fn current(&self) -> eyre::Result<CurrentConditions>;
    fn query_pairs(&self) -> Vec<(&str, &str)>;
}

#[derive(Clone, Debug)]
pub enum Provider {
    WeatherAPI(String),
    OpenMeteo,
}

impl CurrentConditions {
    pub fn get(
        provider: Provider,
        unit: crate::Unit,
        latitude: &str,
        longitude: &str,
    ) -> eyre::Result<CurrentConditions> {
        let client: Box<dyn WeatherProvider> = match provider {
            Provider::WeatherAPI(key) => Box::new(weather_api::Client::new(
                key,
                latitude.to_string(),
                longitude.to_string(),
            )),
            Provider::OpenMeteo => Box::new(open_meteo::Client::new(
                unit,
                latitude.to_string(),
                longitude.to_string(),
            )),
        };

        client.current()
    }

    pub fn set_icon(&mut self, value: String) {
        self.icon = Some(value);
    }
}

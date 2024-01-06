use serde::Serialize;

use crate::{
    cache::Cache, config::Config, location, weather::CurrentConditions,
};

use crate::Unit;

#[derive(Debug, Serialize)]
pub struct Output {
    pub temp: i32,
    pub icon: String,
}

pub struct Conditions {
    config: Config,
    region: Option<String>,
}

impl Conditions {
    #[must_use]
    pub fn new(config: Config, region: Option<String>) -> Self {
        Self { config, region }
    }

    /// Fetches current weather conditions based on the provided configuration
    /// and location.
    ///
    /// # Arguments
    ///
    /// * `cache` - A mutable reference to the cache object used for storing
    ///   location data.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the fetched weather conditions as an
    /// `Output` struct on success, or an `eyre::Error` on failure.
    ///
    /// # Errors
    ///
    /// This function can return an `eyre::Error` if any of the following
    /// conditions are met:
    ///
    /// * The location retrieval from the cache fails.
    /// * The location retrieval from the configuration fails.
    /// * The retrieval of current weather conditions fails.
    pub async fn fetch(&mut self, cache: &mut Cache) -> eyre::Result<Output> {
        let location = if let Some(region) = &self.region {
            location::get(cache, Some(region)).await?
        } else {
            self.config.get_location(cache).await?
        };

        let conditions = CurrentConditions::get(&self.config, &location)?;
        let output = self.to_output(conditions);

        Ok(output)
    }

    fn to_output(&self, conditions: CurrentConditions) -> Output {
        let temp = match self.config.unit {
            Unit::C => conditions.temp_c,
            Unit::F => conditions.temp_f,
        };

        #[allow(clippy::cast_possible_truncation)]
        let temp = temp as i32;

        let icon = conditions.icon;

        Output { temp, icon }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_conditions_to_output() {
        let config = Config {
            unit: Unit::C,
            ..Default::default()
        };
        let conditions = CurrentConditions {
            temp_c: 10.0,
            temp_f: 50.0,
            icon: "icon".to_string(),
        };

        let output = Conditions::new(config, None).to_output(conditions);

        assert_eq!(output.temp, 10);
        assert_eq!(output.icon, "icon");
    }
}

use serde::Serialize;

use crate::{
    args::Unit, cache::Cache, config::Config, location,
    weather::CurrentConditions,
};

#[derive(Debug, Serialize)]
struct Output {
    temp: i32,
    icon: String,
}

pub struct Conditions {
    config: Config,
    region: Option<String>,
}

impl Conditions {
    pub fn new(config: Config, region: Option<String>) -> Self {
        Self { config, region }
    }

    /// Fetch the current weather conditions given supplied configuration.
    ///
    /// - use configured location or infer location via IP
    /// - retrieve wather conditions from weather provider(s) for location
    /// - compose output structure
    /// - convert output to JSON and return
    pub async fn fetch(&mut self, cache: &mut Cache) -> eyre::Result<String> {
        let location = if let Some(region) = &self.region {
            location::get(cache, Some(region)).await?
        } else {
            self.config.get_location(cache).await?
        };

        let conditions = CurrentConditions::get(&self.config, &location)?;

        #[cfg(feature = "sketchybar")]
        self.notify_sketchybar(&conditions)?;

        let output = self.to_output(conditions);

        Ok(ureq::serde_json::to_string(&output)?)
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

    #[cfg(feature = "sketchybar")]
    fn notify_sketchybar(
        &self,
        conditions: &CurrentConditions,
    ) -> eyre::Result<String> {
        #[allow(clippy::cast_possible_truncation)]
        // TODO: this shouldn't be hard-code. Consider additional CLI args
        let message = format!(
            "--set weather_logo icon=\"{}\" --set weather label=\"{}°{}\"",
            conditions.icon,
            conditions.temp_f as i32,
            self.config.unit.as_char().to_ascii_uppercase()
        );

        match sketchybar_rs::message(&message) {
            Ok(_) => Ok(String::new()),
            Err(err) => Err(eyre::eyre!(err.to_string())),
        }
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

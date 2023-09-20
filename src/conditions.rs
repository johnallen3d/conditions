use serde::Serialize;

use crate::{args::Unit, config::Config, weather::CurrentConditions};

#[derive(Debug, Serialize)]
struct Output {
    temp: i32,
    icon: String,
}

pub struct Conditions {
    config: Config,
}

impl Conditions {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Fetch the current weather conditions given supplied configuration.
    ///
    /// - use configured location or infer location via IP
    /// - retrieve wather conditions from weather provider(s) for location
    /// - compose output structure
    /// - convert output to JSON and return
    pub fn fetch(&self) -> eyre::Result<String> {
        let location = self.config.get_location()?;
        let conditions = CurrentConditions::get(&self.config, &location)?;
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conditions_to_output() {
        let config = Config {
            unit: Unit::C,
            ..Default::default()
        };
        let conditions = CurrentConditions {
            temp_c: 10.0,
            temp_f: 50.0,
            icon: "icon".to_string(),
        };
        let output = Conditions::new(config).to_output(conditions);

        assert_eq!(output.temp, 10);
        assert_eq!(output.icon, "icon");
    }
}

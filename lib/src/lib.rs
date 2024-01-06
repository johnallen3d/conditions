#![deny(clippy::pedantic)]

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize};

pub(crate) mod api;
pub mod cache;
pub mod conditions;
pub mod config;
pub mod icons;
pub mod location;
mod weather;

pub use cache::Cache;
pub use conditions::Conditions;
pub use config::Config;

#[derive(Clone, Copy, Debug, Default, Serialize)]
pub enum Unit {
    C,
    #[default]
    F,
}

impl Unit {
    #[must_use]
    pub fn from_char(unit: char) -> Option<Self> {
        match unit {
            'c' => Some(Self::C),
            'f' => Some(Self::F),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_char(&self) -> char {
        match self {
            Unit::C => 'c',
            Unit::F => 'f',
        }
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text = match self {
            Unit::C => "celsius",
            Unit::F => "fahrenheit",
        };
        write!(f, "{text}")
    }
}

impl<'de> Deserialize<'de> for Unit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?.to_lowercase();
        match s.as_str() {
            "c" => Ok(Unit::C),
            _ => Ok(Unit::F),
        }
    }
}

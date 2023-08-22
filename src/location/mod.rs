use std::fmt;

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub(crate) mod from_ip;
pub(crate) mod from_postal_code;

#[derive(Error, Debug, PartialEq)]
pub enum ParseCoordinatesError {
    #[error("provided string was not in the format 'lat,long'")]
    InvalidFormat,
    #[error("provided postal code was not found")]
    UnknownLocation(String),
}

pub trait LocationProvider {
    fn locate(&self) -> eyre::Result<Location>;
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Location {
    pub loc: String,
    pub latitude: String,
    pub longitude: String,
    pub postal_code: String,
}

impl fmt::Display for Location {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{} - {}", self.loc, self.postal_code)
    }
}

pub fn get(postal_code: Option<&str>) -> eyre::Result<Location> {
    let client: Box<dyn LocationProvider> = if postal_code.is_some() {
        Box::new(from_postal_code::Client::new(postal_code))
    } else {
        Box::new(from_ip::Client::new())
    };

    client.locate()
}

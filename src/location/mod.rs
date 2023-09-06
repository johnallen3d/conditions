use std::fmt;

use eyre::WrapErr;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub(crate) mod from_ip;
pub(crate) mod from_postal_code;
use crate::api::Fetchable;

#[derive(Error, Debug, PartialEq)]
pub enum ParseCoordinatesError {
    #[error("provided string was not in the format")]
    InvalidFormat(String),
    #[error("provided postal code was not found")]
    UnknownLocation(String),
}

pub trait HttpClient<T, U>
where
    for<'de> T: Deserialize<'de>,
    U: From<T>,
{
    fn fetch(&self) -> eyre::Result<U> {
        ureq::get(self.url())
            .query_pairs(self.query_pairs())
            .call()
            .map_err(|_| eyre::eyre!("unknown error"))?
            .into_json::<T>()
            .wrap_err(format!("error parsing response from: {}", self.url()))
            .map(U::from)
    }

    fn url(&self) -> &str;

    fn query_pairs(&self) -> Vec<(&str, &str)>;
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
        write!(
            fmt,
            "Coordinates: {}\n  Postal Code: {}",
            self.loc, self.postal_code,
        )
    }
}

pub fn get(region: Option<&str>) -> eyre::Result<Location> {
    if region.is_some() {
        from_postal_code::Client::new(region)?.fetch()
    } else {
        from_ip::Client::new().fetch()
    }
}

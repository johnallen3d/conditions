use std::fmt;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use thiserror::Error;

use crate::cache::Cache;
pub(crate) mod from_ip;
pub(crate) mod from_postal_code;
use crate::api::Fetchable;

#[derive(Error, Debug, PartialEq)]
pub enum ParseCoordinatesError {
    // #[error("provided string was not in the format")]
    // InvalidFormat(String),
    #[error("provided postal code was not found")]
    UnknownLocation(String),
}

#[derive(FromRow, Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
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

/// Fetches the current location based on either a specified region or the
/// client's IP address.
///
/// This function has two main branches of logic:
///
/// 1. If a `region` is provided:
///    - It first checks the cache to see if the location data for the specified
///      region is already available.
///    - If cached, it logs the cache hit and returns the cached location data.
///    - If not cached, it fetches the location data based on the region's
///      postal code, caches the new data, and returns it.
///
/// 2. If no `region` is provided:
///    - It fetches the location data based on the client's IP address, caches
///      the new data, and returns it.
///
/// # Arguments
///
/// - `cache`: A mutable reference to a `Cache` object for caching location
///   data.
/// - `region`: An `Option<&str>` representing a region (as a postal code). If
///   `None`, the function will use the client's IP address to fetch location
///   data.
///
/// # Returns
///
/// - A `Result` with the `Location` object, or an `eyre::Error` if any step of
///   the process fails.
///
/// # Errors
///
/// This function will return an `eyre::Error` if:
///
/// - The location data cannot be fetched from the cache or the external service.
/// - The cache is unable to store the new location data.
/// - The provided region's postal code is invalid or not supported.
/// - There is a failure in network connectivity or during the external service call.
pub async fn get(
    cache: &mut Cache,
    region: Option<&str>,
) -> eyre::Result<Location> {
    let Some(region) = region else {
        let location = from_ip::Client::new().fetch()?;

        cache.set(&location).await?;

        return Ok(location);
    };

    let Some(location) = cache.get(region).await? else {
        let location = from_postal_code::Client::new(region)?.fetch()?;

        cache.set(&location).await?;

        return Ok(location);
    };

    Ok(location)
}

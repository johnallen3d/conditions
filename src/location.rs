use std::fmt;
use std::str::FromStr;

use thiserror::Error;

const LOCATION_URL: &str = "https://ipinfo.io/loc";

#[derive(Error, Debug, PartialEq)]
pub enum ParseCoordinatesError {
    #[error("provided string was not in the format 'lat,long'")]
    InvalidFormat,
}

pub trait HttpClient {
    fn get_location(&self) -> Result<String, ParseCoordinatesError>;
}

pub struct UreqClient;

impl HttpClient for UreqClient {
    fn get_location(&self) -> Result<String, ParseCoordinatesError> {
        let location = match ureq::get(LOCATION_URL).call() {
            Ok(response) => response.into_string().unwrap_or_default(),
            Err(_) => return Err(ParseCoordinatesError::InvalidFormat),
        };

        Ok(location)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Coordinates {
    latitude: String,
    longitude: String,
}

impl Coordinates {
    fn new(lat: String, long: String) -> Self {
        Self {
            latitude: lat,
            longitude: long,
        }
    }
}

impl FromStr for Coordinates {
    type Err = ParseCoordinatesError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = text.split(',').collect();

        if parts.len() != 2 {
            return Err(ParseCoordinatesError::InvalidFormat);
        }

        let lat = parts[0].trim();
        let long = parts[1].trim();

        Ok(Self::new(lat.to_owned(), long.to_owned()))
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{},{}", self.latitude, self.longitude)
    }
}

pub fn current<T: HttpClient>(client: &T) -> Result<Coordinates, ParseCoordinatesError> {
    let response = client.get_location()?;

    Coordinates::from_str(&response)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockClient;

    impl HttpClient for MockClient {
        fn get_location(&self) -> Result<String, ParseCoordinatesError> {
            Ok(COORDS.to_string())
        }
    }

    const LAT: &str = "40.748817";
    const LONG: &str = "-73.985428";
    const COORDS: &str = "40.748817,-73.985428";

    #[test]
    fn test_current() {
        let mock_client = MockClient;
        let result = current(&mock_client).unwrap();
        assert_eq!(
            result,
            Coordinates {
                latitude: LAT.to_string(),
                longitude: LONG.to_string(),
            }
        );
    }

    #[test]
    fn coordinates_from_str() {
        let coords = Coordinates::from_str(COORDS);

        assert_eq!(
            coords,
            Ok(Coordinates {
                latitude: LAT.to_string(),
                longitude: LONG.to_string(),
            })
        );
    }

    #[test]
    fn coordinates_from_str_invalid() {
        let coords = Coordinates::from_str("invalid");
        assert_eq!(coords.err(), Some(ParseCoordinatesError::InvalidFormat));
    }
}

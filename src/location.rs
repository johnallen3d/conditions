use std::fmt;
use std::str::FromStr;
use std::string::ParseError;

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
    type Err = ParseError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = text.split(',').collect();

        if parts.len() != 2 {
            // TODO
            // Err(Self::Err)?;
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

pub fn current() -> Result<Coordinates, Box<dyn std::error::Error>> {
    let response = ureq::get("https://ipinfo.io/loc").call()?.into_string()?;

    Ok(Coordinates::from_str(&response)?)
}

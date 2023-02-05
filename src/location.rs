use std::fmt;
use std::num::ParseFloatError;
use std::str::FromStr;

#[derive(Debug, Default, PartialEq)]
pub struct Coordinates {
    latitude: f64,
    longitude: f64,
}

impl Coordinates {
    fn new(lat: f64, long: f64) -> Self {
        Self {
            latitude: lat,
            longitude: long,
        }
    }
}

impl FromStr for Coordinates {
    type Err = ParseFloatError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = text.split(',').collect();

        if parts.len() != 2 {
            // TODO
            // Err(Self::Err)?;
        }

        let lat: f64 = parts[0].parse::<f64>()?;
        let long: f64 = parts[1].trim().parse::<f64>()?;

        Ok(Self::new(lat, long))
    }
}

impl fmt::Display for Coordinates {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{},{}", self.latitude, self.longitude)
    }
}

pub fn current() -> Result<Coordinates, ParseFloatError> {
    Ok(Coordinates::new(34.9249, -81.0251))

    // let response = ureq::get("https://ipinfo.io/loc")
    //     .call()
    //     .unwrap()
    //     .into_string()
    //     .unwrap();

    // Coordinates::from_str(&response)
}

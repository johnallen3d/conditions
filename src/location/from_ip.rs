use serde::{Deserialize, Serialize};

use super::Location;

// {
//   "ip": "75.189.252.56",
//   "hostname": "cpe-75-189-252-56.nc.res.rr.com",
//   "city": "Charlotte",
//   "region": "North Carolina",
//   "country": "US",
//   "loc": "35.1287,-80.9338",
//   "org": "AS11426 Charter Communications Inc",
//   "postal": "28273",
//   "timezone": "America/New_York",
//   "readme": "https://ipinfo.io/missingauth"
// }

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Response {
    loc: String,
    postal: String,
}

impl From<Response> for Location {
    fn from(response: Response) -> Self {
        // could use error handling here but `loc` is coming from ipinfo.io
        let (lat, lon) = response.loc.split_once(',').unwrap();

        Self {
            loc: response.loc.clone(),
            latitude: lat.to_string(),
            longitude: lon.to_string(),
            postal_code: response.postal,
        }
    }
}

pub struct Client;

impl Client {
    pub fn new() -> Self {
        Self {}
    }
}

impl crate::api::Fetchable<Response, Location> for Client {
    fn url(&self) -> &'static str {
        "https://ipinfo.io/json"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_from() {
        let lat = "35.12345";
        let lon = "-80.54321";
        let postal_code = "10001";
        let loc = format!("{},{}", lat, lon);

        let response = Response {
            loc: loc.to_string(),
            postal: postal_code.to_string(),
        };

        let location = Location::from(response);

        assert_eq!(location.loc, loc);
        assert_eq!(location.latitude, lat);
        assert_eq!(location.longitude, lon);
        assert_eq!(location.postal_code, postal_code);
    }
}

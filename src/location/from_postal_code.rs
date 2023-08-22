use eyre::WrapErr;
use serde::{Deserialize, Serialize};

use super::*;

// https://nominatim.openstreetmap.org/search.php?format=jsonv2&postalcode=29715
// [
//   {
//     "place_id": 331067798,
//     "licence": "Data © OpenStreetMap contributors, ODbL 1.0. http://osm.org/copyright",
//     "lat": "35.010773338125006",
//     "lon": "-80.91695729385417",
//     "category": "place",
//     "type": "postcode",
//     "place_rank": 21,
//     "importance": 0.22500009999999993,
//     "addresstype": "postcode",
//     "name": "29715",
//     "display_name": "York County, South Carolina, 29715, United States",
//     "boundingbox": [
//       "34.8507733",
//       "35.1707733",
//       "-81.0769573",
//       "-80.7569573"
//     ]
//   }
// ]
pub const URL: &str =
    "https://nominatim.openstreetmap.org/search.php?format=jsonv2&postalcode=";

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
struct Response {
    lat: String,
    lon: String,
    name: String,
}

impl From<Response> for Location {
    fn from(response: Response) -> Self {
        Self {
            loc: format!("{},{}", response.lat, response.lon),
            latitude: response.lat.clone(),
            longitude: response.lon.clone(),
            postal_code: response.name,
        }
    }
}

pub struct Client {
    postal_code: String,
}

impl Client {
    pub fn new(postal_code: Option<&str>) -> Self {
        Self {
            postal_code: postal_code.unwrap().to_string(),
        }
    }
}

impl LocationProvider for Client {
    fn locate(&self) -> eyre::Result<Location> {
        let url = format!("{}{}", URL, self.postal_code);

        ureq::get(&url)
            .call()
            .map_err(|_| ParseCoordinatesError::InvalidFormat)
            .wrap_err("error getting location from postal code")?
            .into_json::<Vec<Response>>()
            .wrap_err("error parsing response into Vec<Response>")?
            .first()
            .ok_or_else(|| {
                ParseCoordinatesError::UnknownLocation(
                    "error getting location from postal code".to_string(),
                )
            })
            .wrap_err("error getting the first element from response")
            .map(|location| Location::from(location.to_owned()))
    }
}

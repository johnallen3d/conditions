use eyre::WrapErr;
use serde::{Deserialize, Serialize};

use super::{Location, ParseCoordinatesError};

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

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Response {
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
    query: Vec<(String, String)>,
}

impl Client {
    pub fn new(region: Option<&str>) -> eyre::Result<Self> {
        let (postal_code, country) =
            region.unwrap().split_once(',').ok_or_else(|| {
                eyre::eyre!(ParseCoordinatesError::InvalidFormat(
                    "missing comma or text before/after comma".to_string(),
                ))
            })?;

        if postal_code.is_empty() {
            Err(eyre::eyre!(ParseCoordinatesError::InvalidFormat(
                "missing postal code".to_string()
            )))?;
        }

        if country.is_empty() {
            Err(eyre::eyre!(ParseCoordinatesError::InvalidFormat(
                "missing country".to_string()
            )))?;
        }

        Ok(Self {
            query: vec![
                ("format".to_string(), "json".to_string()),
                ("postalcode".to_string(), postal_code.to_string()),
                ("country".to_string(), country.to_string()),
            ],
        })
    }
}

impl crate::api::Fetchable<Response, Location> for Client {
    const URL: &'static str = "https://nominatim.openstreetmap.org/search.php";

    fn fetch(&self) -> eyre::Result<Location> {
        ureq::get(Self::URL)
            .query_pairs(self.query_pairs())
            .call()
            .map_err(|_| eyre::eyre!("unknown error"))
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

    fn query(&self) -> Option<&Vec<(String, String)>> {
        Some(&self.query)
    }
}

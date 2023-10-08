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

#[derive(Debug)]
pub struct Client {
    query: Vec<(String, String)>,
}

impl Client {
    pub fn new(region: &str) -> eyre::Result<Self> {
        let (postal_code, country) =
            region.split_once(',').ok_or_else(|| {
                eyre::eyre!("invalid location, expect [POSTAL_CODE, COUNTRY]"
                    .to_string(),)
            })?;

        if postal_code.is_empty() {
            return Err(eyre::eyre!("missing postal code".to_string()));
        }

        if country.is_empty() {
            return Err(eyre::eyre!("missing country".to_string()));
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
    fn url(&self) -> &'static str {
        "https://nominatim.openstreetmap.org/search.php"
    }

    fn fetch(&self) -> eyre::Result<Location> {
        ureq::get(self.url())
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
            .map(|location| Location::from(location.clone()))
    }

    fn query(&self) -> Option<&Vec<(String, String)>> {
        Some(&self.query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_from() {
        let lat = "35.12345";
        let long = "-80.54321";
        let name = "10001";
        let loc = format!("{lat},{long}");

        let response = Response {
            lat: lat.to_string(),
            lon: long.to_string(),
            name: name.to_string(),
        };

        let location = Location::from(response);

        assert_eq!(location.loc, loc);
        assert_eq!(location.latitude, lat);
        assert_eq!(location.longitude, long);
        assert_eq!(location.postal_code, name);
    }

    #[test]
    fn test_client_new_valid() {
        let result = Client::new("10001,US").unwrap();
        let query = vec![
            ("format".to_string(), "json".to_string()),
            ("postalcode".to_string(), "10001".to_string()),
            ("country".to_string(), "US".to_string()),
        ];

        assert_eq!(result.query, query);
    }

    #[test]
    fn test_client_new_invalid_format() {
        let result = Client::new("12345");

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("invalid location, expect [POSTAL_CODE, COUNTRY]"));
    }

    #[test]
    fn test_client_new_missing_postal_code() {
        let result = Client::new(",CA");

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("missing postal code"));
    }

    #[test]
    fn test_client_new_missing_country() {
        let result = Client::new("12345,");

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("missing country"));
    }
}

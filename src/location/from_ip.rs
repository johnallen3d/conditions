use eyre::WrapErr;

use super::*;

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
pub const URL: &str = "https://ipinfo.io/json";

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
struct Response {
    loc: String,
    postal: String,
}

impl From<Response> for Location {
    fn from(response: Response) -> Self {
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

impl LocationProvider for Client {
    fn locate(&self) -> eyre::Result<Location> {
        ureq::get(URL)
            .query_pairs(self.query_pairs())
            .call()
            .map_err(|_| eyre::eyre!("unknown error"))?
            .into_json::<Response>()
            .wrap_err("error parsing response from ipinfo.io")
            .map(Location::from)
    }

    fn query_pairs(&self) -> Vec<(&str, &str)> {
        vec![]
    }
}

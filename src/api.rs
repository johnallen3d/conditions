use eyre::WrapErr;
use serde::Deserialize;

/// The `Fetchable` trait provides a generalized interface for making basic HTTP
/// requests.
///
/// Implementations are expected to specify the `URL` constant and can optionally
/// provide custom query parameters.
///
/// Types `T` and `U` are involved in deserialization and conversion.
/// - `T` is the type that the HTTP response can be deserialized into.
/// - `U` is the type that the deserialized response will be converted into.
///
/// # Examples
///
/// ```ignore
/// use crate::api::Fetchable;
///
/// struct SearchClient {
///     query: Vec<(String, String)>
/// }
///
/// impl SearchClient {
///     pub fn new(q: String) -> Self {
///         Self { query: vec!["q".to_string(), q] }
///     }
/// }
///
/// impl Fetchable<YourResponseType, YourOutputType> for SearchClient {
///     const URL: &'static str = "https://your.api/endpoint";
///
///     fn query(&self) -> Option<&Vec<(String, String)>> {
///         Some(&self.query)
///     }
/// }
/// ```
///
/// # Errors
///
/// Returns `eyre::Result` which is a custom type alias over standard `Result`.
/// Typically, errors will be of type `eyre::Report`.
pub trait Fetchable<T, U>
where
    for<'de> T: Deserialize<'de>,
    U: From<T>,
{
    /// The API endpoint to fetch data from.
    const URL: &'static str;

    /// Makes an HTTP GET request to fetch data from the API endpoint.
    ///
    /// # Returns
    ///
    /// Returns `eyre::Result<U>` where `U` is the type that the deserialized
    /// response will be converted into.
    fn fetch(&self) -> eyre::Result<U> {
        ureq::get(Self::URL)
            .query_pairs(self.query_pairs())
            .call()
            .map_err(|_| eyre::eyre!("unknown error"))?
            .into_json::<T>()
            .wrap_err(format!("error parsing response from: {}", Self::URL))
            .map(U::from)
    }

    /// Returns optional query parameters as a vector of key-value pairs.
    ///
    /// # Returns
    ///
    /// An optional reference to a vector of key-value pairs `(String, String)`.
    fn query(&self) -> Option<&Vec<(String, String)>> {
        None
    }

    /// Helper method to convert query parameters into a format suitable for
    /// HTTP requests.
    ///
    /// # Returns
    ///
    /// A vector of key-value pairs as `(&str, &str)`.
    fn query_pairs(&self) -> Vec<(&str, &str)> {
        if let Some(query) = self.query() {
            query
                .iter()
                .map(|(k, v)| (k.as_str(), v.as_str()))
                .collect()
        } else {
            vec![]
        }
    }
}

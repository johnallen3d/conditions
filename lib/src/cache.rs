use std::env;

use eyre::WrapErr;
use sqlx::{Connection, SqliteConnection};

use crate::location::Location;

#[derive(Debug)]
pub struct Cache {
    connection: SqliteConnection,
}

impl Cache {
    /// Creates a new instance of the `Cache` struct.
    ///
    /// # Arguments
    ///
    /// * `path` - An optional `String` representing the path to the `SQLite`
    /// database file. If `Some`, the provided path will be used. If `None`, the
    /// `DATABASE_URL` environment variable will be used as the path. If the
    /// `DATABASE_URL` environment variable is not set, an `eyre::Report` error
    /// will be returned.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the newly created `Cache` instance on
    /// success, or an `eyre::Report` error on failure.
    ///
    /// # Errors
    ///
    /// This function can return the following errors:
    ///
    /// * `eyre::Report` - If the `DATABASE_URL` environment variable is not set
    /// and no `path` is provided.
    /// * `sqlx::Error` - If there is an error connecting to the `SQLite`
    /// database or executing the SQL query to create the `cache` table.
    pub async fn new(path: Option<String>) -> eyre::Result<Self> {
        let path = match path {
            Some(path) => path,
            None => {
                env::var("DATABASE_URL").wrap_err("DATABASE_URL not set")?
            }
        };

        let db_url = format!("sqlite://{path}?mode=rwc");
        let mut connection = SqliteConnection::connect(&db_url).await?;

        sqlx::query(
            r"
            CREATE TABLE IF NOT EXISTS cache (
                postal_code TEXT NOT NULL,
                loc         TEXT NOT NULL,
                latitude    TEXT NOT NULL,
                longitude   TEXT NOT NULL,

                UNIQUE(postal_code)
            )
            ",
        )
        .execute(&mut connection)
        .await?;

        Ok(Self { connection })
    }

    /// Caches the provided location.
    ///
    /// This function takes a mutable reference to `self` (the cache) and a
    /// reference to a `Location` struct.
    /// It inserts or updates the location in the cache table based on the
    /// postal code.
    ///
    /// # Arguments
    ///
    /// * `location` - A reference to a `Location` struct containing the details
    /// of the location to be inserted or updated.
    ///
    /// # Returns
    ///
    /// This function returns a `Result` indicating success or failure. If the
    /// operation is successful, it returns `Ok(())`.
    /// If an error occurs during the execution of the SQL query or the database
    /// connection, it returns an `eyre::Result` with the error details.
    ///
    /// # Errors
    ///
    /// This function can raise an error if there is an issue with the SQL query
    /// execution or the database connection.
    /// The specific error type is `eyre::Result`, which provides a flexible way
    /// to handle and propagate errors.
    pub async fn set(&mut self, location: &Location) -> eyre::Result<()> {
        let query = r"
            INSERT INTO cache (
               postal_code
              ,loc
              ,latitude
              ,longitude
            )
            VALUES (
               ?
              ,?
              ,?
              ,?
            )
            ON CONFLICT(postal_code)
            DO UPDATE SET
               loc = excluded.loc
              ,latitude = excluded.latitude
              ,longitude = excluded.longitude
            ;
        ";

        sqlx::query(query)
            .bind(&location.postal_code)
            .bind(&location.loc)
            .bind(&location.latitude)
            .bind(&location.longitude)
            .execute(&mut self.connection)
            .await?;

        Ok(())
    }

    /// Retrieves a location from the cache based on the provided postal code.
    ///
    /// # Arguments
    ///
    /// * `postal_code` - A string representing the postal code of the location
    /// to retrieve.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Location>, eyre::Report>` - A result that contains an
    /// optional `Location` if found in the cache, or an `eyre::Report` if an
    /// error occurred.
    ///
    /// # Errors
    ///
    /// This function can return an `eyre::Report` if there was an error
    /// executing the database query or fetching the location from the cache.
    pub async fn get(
        &mut self,
        postal_code: &str,
    ) -> eyre::Result<Option<Location>> {
        let query = "SELECT * FROM cache WHERE postal_code = ?;";

        let location: Option<Location> = sqlx::query_as(query)
            .bind(postal_code)
            .fetch_optional(&mut self.connection)
            .await?;

        Ok(location)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_cache() {
        let dir = tempdir().unwrap();
        let db_path = dir.path().join("test.db");

        let mut cache = Cache::new(Some(db_path.display().to_string()))
            .await
            .unwrap();

        let location = Location {
            postal_code: "12345".to_string(),
            loc: "Test Location".to_string(),
            latitude: "12.345".to_string(),
            longitude: "54.321".to_string(),
        };

        cache.set(&location).await.unwrap();

        let retrieved_location =
            cache.get(&location.postal_code).await.unwrap();

        assert_eq!(location, retrieved_location.unwrap());

        assert!(cache.get("00000").await.unwrap().is_none());

        drop(dir);
    }
}

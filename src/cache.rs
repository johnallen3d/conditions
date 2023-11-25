use std::env;

use eyre::WrapErr;
use sqlx::{Connection, SqliteConnection};

use crate::location::Location;

#[derive(Debug)]
pub struct Cache {
    connection: SqliteConnection,
}

impl Cache {
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

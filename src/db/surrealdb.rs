use crate::models::depth_history_model::DepthHistory;
use crate::models::rune_pool_history_model::RunePoolHistory;
use crate::utils::types::{Error, Result};
use dotenv::dotenv;
use std::env;
use surrealdb::engine::any::{self, Any};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use surrealdb::{self};

pub struct SurrealDBWrapper {
    pub db: Surreal<Any>,
}

impl SurrealDBWrapper {
    pub async fn init() -> Result<Self> {
        dotenv().ok();

        let surrealdb_url = match env::var("SURREAL_DATABASE_URL") {
            Ok(v) => v.to_string(),
            Err(_) => {
                return Err(Error::DataBaseConnectionFailed(
                    "Unable to read SurrealDB URI.".to_string(),
                ))
            }
        };

        let db = any::connect(surrealdb_url).await.map_err(|e| {
            eprintln!("Error connecting to SurrealDB: {:?}", e);
            Error::DataBaseConnectionFailed(
                "Failed to connect to SurrealDB. Check the database URL.".to_string(),
            )
        })?;

        db.signin(Root {
            username: "azeem",
            password: "azeem@4659",
        })
        .await
        .map_err(|e| {
            eprintln!("Error signing in to SurrealDB as root: {:?}", e);
            Error::DataBaseConnectionFailed(
                "Failed to log in as root user to SurrealDB.".to_string(),
            )
        })?;

        db.use_ns("azeem-0")
            .use_db("database-metrics-testing")
            .await
            .map_err(|e| {
                eprintln!(
                    "Error selecting namespace or database in SurrealDB: {:?}",
                    e
                );
                Error::DataBaseConnectionFailed(
                    "Unable to connect to the specified namespace or database in SurrealDB."
                        .to_string(),
                )
            })?;

        let surrealdb = SurrealDBWrapper { db };

        Ok(surrealdb)
    }

    pub async fn insert_depth_history(&self, depth_history: &DepthHistory) -> Result<()> {
        let _inserted_record: Vec<DepthHistory> = self
            .db
            .insert("depth_history")
            .content(depth_history.clone())
            .await
            .map_err(|e| {
                eprintln!("Error inserting depth history into SurrealDB: {:?}", e);
                Error::DataBaseInsertionFailed(
                    "Failed to insert depth history data into SurrealDB.".to_string(),
                )
            })?;

        Ok(())
    }

    pub async fn insert_rune_pool_history(
        &self,
        rune_pool_history: &RunePoolHistory,
    ) -> Result<()> {
        let _inserted_record: Vec<RunePoolHistory> = self
            .db
            .insert("rune_pool_history")
            .content(rune_pool_history.clone())
            .await
            .map_err(|e| {
                eprintln!("Error inserting rune pool history into SurrealDB: {:?}", e);
                Error::DataBaseInsertionFailed(
                    "Failed to insert rune pool history data into SurrealDB.".to_string(),
                )
            })?;

        Ok(())
    }

    pub async fn read_depth_history(&self) -> Result<Vec<DepthHistory>> {
        let depth_history: Vec<DepthHistory> =
            self.db.select("depth_history").await.map_err(|e| {
                eprintln!(
                    "Error retrieving depth history data from SurrealDB: {:?}",
                    e
                );
                Error::DataBaseReadFailed(
                    "Failed to retrieve depth history data from SurrealDB.".to_string(),
                )
            })?;

        Ok(depth_history)
    }

    pub async fn read_rune_pool_history(&self) -> Result<Vec<RunePoolHistory>> {
        let rune_pool_history: Vec<RunePoolHistory> =
            self.db.select("rune_pool_history").await.map_err(|e| {
                eprintln!(
                    "Error retrieving rune pool history data from SurrealDB: {:?}",
                    e
                );
                Error::DataBaseReadFailed(
                    "Failed to retrieve rune pool history data from SurrealDB.".to_string(),
                )
            })?;

        Ok(rune_pool_history)
    }
}

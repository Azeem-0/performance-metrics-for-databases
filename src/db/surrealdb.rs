use crate::models::depth_history_model::DepthHistory;
use crate::models::rune_pool_history_model::RunePoolHistory;
use crate::utils::types::{Error, Result};
use dotenv::dotenv;
use std::env;
use surrealdb::engine::any::{self, Any};
use surrealdb::engine::remote::ws::{Client, Ws};
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
                    "Unable to read surreal db uri.".to_string(),
                ))
            }
        };

        let db = any::connect(surrealdb_url).await.map_err(|e| {
            println!("{:?}", e);
            Error::DataBaseConnectionFailed("Invalid database URL".to_string())
        })?;

        db.signin(Root {
            username: "azeem",
            password: "azeem@4659",
        })
        .await
        .map_err(|e| {
            println!("{:?}", e);
            Error::DataBaseConnectionFailed("Cannot log in as root user".to_string())
        })?;

        db.use_ns("azeem-0")
            .use_db("database-metrics-testing")
            .await
            .map_err(|e| {
                Error::DataBaseConnectionFailed("Unable to connect to the database".to_string())
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
                eprintln!("Error inserting depth history: {:?}", e);
                Error::DataBaseConnectionFailed("Insertion failed".to_string())
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
                eprintln!("Error inserting rune pool history into surreal db: {:?}", e);
                Error::DataBaseConnectionFailed(
                    "Insertion failed for rune pool data in surreal db".to_string(),
                )
            })?;

        Ok(())
    }

    pub async fn read_depth_history(&self) -> Result<Vec<DepthHistory>> {
        let depth_history: Vec<DepthHistory> =
            self.db.select("depth_history").await.map_err(|e| {
                eprintln!(
                    "Error retrieveing depth history history from surreal db: {:?}",
                    e
                );
                Error::DataBaseError(
                    "Retrieval failed for depth history data in surreal db".to_string(),
                )
            })?;

        Ok(depth_history)
    }
    pub async fn read_rune_pool_history(&self) -> Result<Vec<RunePoolHistory>> {
        let depth_history: Vec<RunePoolHistory> =
            self.db.select("rune_pool_history").await.map_err(|e| {
                eprintln!(
                    "Error retrieveing rune pool history history from surreal db: {:?}",
                    e
                );
                Error::DataBaseError(
                    "Retrieval failed for rune pool history data in surreal db".to_string(),
                )
            })?;
        Ok(depth_history)
    }
}

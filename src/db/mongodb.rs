use dotenv::dotenv;
use std::env;

use mongodb::{results::InsertOneResult, Client, Collection};

use crate::{
    models::{
        depth_history_model::DepthHistory, rune_pool_history_model::RunePoolHistory,
        swaps_history_model::SwapsHistory,
    },
    utils::types::{Error, Result},
};

pub struct MongoDB {
    pub depth_history: Collection<DepthHistory>,
    pub swaps_history: Collection<SwapsHistory>,
    pub rune_pule_history: Collection<RunePoolHistory>,
}

impl MongoDB {
    pub async fn init() -> Result<Self> {
        dotenv().ok();
        let uri = match env::var("MONGO_DATABASE_URL") {
            Ok(v) => v.to_string(),
            Err(_) => {
                return Err(Error::DataBaseConnectionFailed(
                    "Unable to read mongodb uri.".to_string(),
                ))
            }
        };

        let client: Option<Client> = Some(Client::with_uri_str(uri).await.unwrap());

        let client = match client {
            Some(clt) => clt,
            None => {
                return Err(Error::DataBaseConnectionFailed(
                    "Failed connecting to the mongodb client, check the mongouri.".to_string(),
                ))
            }
        };

        let db = client.database("database_metrics");

        let depth_history_collection: Collection<DepthHistory> = db.collection("depth_history");

        let swaps_history_collection: Collection<SwapsHistory> = db.collection("swaps_history");

        let rune_pool_collection: Collection<RunePoolHistory> = db.collection("rune_pool_history");

        Ok(MongoDB {
            depth_history: depth_history_collection,
            swaps_history: swaps_history_collection,
            rune_pule_history: rune_pool_collection,
        })
    }

    pub async fn insert_depth_history(
        &self,
        depth_history: DepthHistory,
    ) -> Result<InsertOneResult> {
        let insert_result: InsertOneResult = self
            .depth_history
            .insert_one(depth_history)
            .await
            .expect("Faild to insert depth history");

        Ok(insert_result)
    }

    pub async fn insert_swaps_history(
        &self,
        swaps_history: SwapsHistory,
    ) -> Result<InsertOneResult> {
        let insert_result = self
            .swaps_history
            .insert_one(swaps_history)
            .await
            .expect("Failed to insert swaps history");

        Ok(insert_result)
    }
}

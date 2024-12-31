use rusty_leveldb::{AsyncDB, Options};
use std::sync::Arc;

use crate::{
    models::depth_history_model::DepthHistory,
    utils::types::{Error, Result},
};

pub struct LevelDB {
    db: Arc<AsyncDB>,
}

impl LevelDB {
    pub async fn init() -> Result<Self> {
        let opt = Options::default();
        let db = AsyncDB::new("data/leveldb", opt).map_err(|e| {
            Error::DataBaseConnectionFailed(format!("Failed to initialize LevelDB: {:?}", e))
        })?;

        Ok(LevelDB { db: Arc::new(db) })
    }

    pub async fn insert_data(&self, key: Vec<u8>, val: Vec<u8>) -> Result<()> {
        self.db.put(key.clone(), val).await.map_err(|e| {
            Error::DataBaseInsertionFailed(format!("Failed to insert data into LevelDB: {:?}", e))
        })?;

        let mut keys_index: Vec<Vec<u8>> = match self.db.get(b"_keys_index".to_vec()).await {
            Ok(Some(index)) => serde_json::from_slice(&index).map_err(|e| {
                Error::DataBaseReadFailed(format!(
                    "Failed to deserialize keys index from LevelDB: {:?}",
                    e
                ))
            })?,
            Ok(None) => vec![],
            Err(e) => {
                return Err(Error::DataBaseReadFailed(format!(
                    "Failed to retrieve keys index from LevelDB: {:?}",
                    e
                )))
            }
        };

        keys_index.push(key.clone());
        self.db
            .put(
                b"_keys_index".to_vec(),
                serde_json::to_vec(&keys_index).map_err(|e| {
                    Error::DataBaseInsertionFailed(format!(
                        "Failed to serialize updated keys index for LevelDB: {:?}",
                        e
                    ))
                })?,
            )
            .await
            .map_err(|e| {
                Error::DataBaseInsertionFailed(format!(
                    "Failed to update keys index in LevelDB: {:?}",
                    e
                ))
            })?;

        Ok(())
    }

    pub async fn read_data(&self) -> Result<()> {
        let keys_index: Vec<Vec<u8>> = match self.db.get(b"_keys_index".to_vec()).await {
            Ok(Some(index)) => serde_json::from_slice(&index).map_err(|e| {
                Error::DataBaseReadFailed(format!(
                    "Failed to deserialize keys index from LevelDB: {:?}",
                    e
                ))
            })?,
            Ok(None) => {
                println!("No data found in LevelDB!");
                return Ok(());
            }
            Err(e) => {
                return Err(Error::DataBaseReadFailed(format!(
                    "Failed to retrieve keys index from LevelDB: {:?}",
                    e
                )))
            }
        };

        for key in keys_index {
            self.db.get(key.clone()).await.map_err(|e| {
                Error::DataBaseReadFailed(format!(
                    "Failed to retrieve value for key {:?} from LevelDB: {:?}",
                    String::from_utf8_lossy(&key),
                    e
                ))
            })?;
        }

        Ok(())
    }
}

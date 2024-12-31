use rocksdb::DB;

use crate::{
    models::{depth_history_model::DepthHistory, rune_pool_history_model::RunePoolHistory},
    utils::types::{Error, Result},
};

pub struct RocksDB {
    pub db: DB,
}

impl RocksDB {
    pub async fn init() -> Result<Self> {
        let db = DB::open_default("data/rocksdb").map_err(|e| {
            Error::DataBaseConnectionFailed(format!("Failed to open RocksDB: {:?}", e))
        })?;
        Ok(RocksDB { db })
    }

    pub async fn insert_depth_history(&self, depth_history: &DepthHistory) -> Result<()> {
        let key = serde_json::to_vec(&depth_history.start_time).map_err(|e| {
            Error::DataBaseConnectionFailed(format!("Failed to serialize key in RocksDB: {:?}", e))
        })?;
        let val = serde_json::to_vec(depth_history).map_err(|e| {
            Error::DataBaseConnectionFailed(format!(
                "Failed to serialize value in RocksDB: {:?}",
                e
            ))
        })?;

        self.db.put(key, val).map_err(|e| {
            Error::DataBaseInsertionFailed(format!(
                "Failed to insert depth history in RocksDB: {:?}",
                e
            ))
        })?;

        Ok(())
    }

    pub async fn insert_rune_pool_history(
        &self,
        rune_pool_history: &RunePoolHistory,
    ) -> Result<()> {
        let key = serde_json::to_vec(&rune_pool_history.start_time).map_err(|e| {
            Error::DataBaseInsertionFailed(format!("Failed to serialize key in RocksDB: {:?}", e))
        })?;
        let val = serde_json::to_vec(rune_pool_history).map_err(|e| {
            Error::DataBaseInsertionFailed(format!("Failed to serialize value in RocksDB: {:?}", e))
        })?;

        self.db.put(key, val).map_err(|e| {
            Error::DataBaseInsertionFailed(format!(
                "Failed to insert rune pool history in RocksDB: {:?}",
                e
            ))
        })?;

        Ok(())
    }

    pub async fn read_data(&self) -> Result<()> {
        let iter = self.db.iterator(rocksdb::IteratorMode::Start);

        for item in iter {
            let (key, _) = item.map_err(|e| {
                Error::DataBaseReadFailed(format!("Failed to read item in RocksDB: {:?}", e))
            })?;

            let key_str = String::from_utf8(key.to_vec()).map_err(|e| {
                Error::DataBaseReadFailed(format!("Failed to parse key in RocksDB: {:?}", e))
            })?;

            let val = self.db.get(&key_str).map_err(|e| {
                Error::DataBaseReadFailed(format!("Failed to retrieve value in RocksDB: {:?}", e))
            })?;
        }

        Ok(())
    }
}

use rusty_leveldb::{AsyncDB, DBIterator, LdbIterator, Options};
use std::{path::Path, sync::Arc};

use crate::{
    models::{depth_history_model::DepthHistory, rune_pool_history_model::RunePoolHistory},
    utils::types::{Error, Result},
};
pub struct LevelDB {
    db: Arc<AsyncDB>,
}

impl LevelDB {
    pub async fn init() -> Result<Self> {
        let opt = Options::default();
        let db = AsyncDB::new("leveldb/data", opt).unwrap();

        Ok(LevelDB { db: Arc::new(db) })
    }

    pub async fn insert_depth_history(&self, depth_history: &DepthHistory) -> Result<()> {
        let val = serde_json::to_vec(depth_history).unwrap();

        let key = serde_json::to_vec(&depth_history.start_time).unwrap();

        self.db.put(key, val).await.unwrap();

        Ok(())
    }

    pub async fn insert_rune_pool_history(
        &self,
        rune_pool_history: &RunePoolHistory,
    ) -> Result<()> {
        let val = serde_json::to_vec(rune_pool_history).unwrap();

        let key = serde_json::to_vec(&rune_pool_history.start_time).unwrap();

        self.db.put(key, val).await.unwrap();

        Ok(())
    }
}

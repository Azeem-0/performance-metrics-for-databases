use crate::utils::types::{Error, Result};
use leveldb::LevelDB;
use mongodb::MongoDB;
use postgres::PostgreSQL;
use surrealdb::SurrealDBWrapper;

pub mod leveldb;
pub mod mongodb;
pub mod postgres;
pub mod surrealdb;

pub struct DataBases {
    pub mongodb: MongoDB,
    pub postgres: PostgreSQL,
    pub surrealdb: SurrealDBWrapper,
    pub leveldb: LevelDB,
}

pub async fn init_databases() -> Result<DataBases> {
    let mongodb = MongoDB::init().await.map_err(|e| e)?;
    let postgres = PostgreSQL::init().await.map_err(|e| e)?;
    let surrealdb = SurrealDBWrapper::init().await.map_err(|e| e)?;
    let leveldb = LevelDB::init().await.map_err(|e| e)?;

    Ok(DataBases {
        mongodb,
        postgres,
        surrealdb,
        leveldb,
    })
}

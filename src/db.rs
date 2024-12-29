use crate::utils::types::{Error, Result};
use mongodb::MongoDB;
use postgres::PostgreSQL;

pub mod mongodb;
pub mod postgres;

pub struct DataBases {
    pub mongodb: MongoDB,
    pub postgres: PostgreSQL,
}

// pub async fn init_databases() -> Result<DataBases> {
//     let _mongodb = MongoDB::init().await.map_err(|e| e)?;
//     Ok(DataBases{
//         mongodb:_mongodb,
//     })
// }

pub async fn init_databases() -> Result<DataBases> {
    let mongodb = MongoDB::init().await.map_err(|e| e)?;
    let postgres = PostgreSQL::init().await.map_err(|e| e)?;

    Ok(DataBases { mongodb, postgres })
}

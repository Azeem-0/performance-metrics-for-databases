use crate::utils::types::{Error, Result};
use mongodb::MongoDB;

pub mod mongodb;

pub async fn init_databases() -> Result<MongoDB> {
    let _mongodb = MongoDB::init().await.map_err(|e| e)?;
    Ok(_mongodb)
}

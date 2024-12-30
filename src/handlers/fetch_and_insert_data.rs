use std::{sync::Arc, time::Instant};

use crate::{
    db::{
        leveldb::LevelDB, mongodb::MongoDB, postgres::PostgreSQL, surrealdb::SurrealDBWrapper,
        DataBases,
    },
    metrics::performance_metrics::performance_metrics,
    models::{
        depth_history_model::DepthHistoryResponse, rune_pool_history_model::RunePoolHistoryResponse,
    },
    utils::types::{Error, Result},
};
use axum::{response::IntoResponse, Extension};
use reqwest::StatusCode;

async fn insert_depth_history_into_mongodb(
    mongodb: &MongoDB,
    resp: &DepthHistoryResponse,
) -> Result<bool> {
    let start_time = Instant::now();
    for depth_history in &resp.intervals {
        if let Err(err) = mongodb.insert_depth_history(depth_history).await {
            dbg!("Failed to insert depth history data into database");
            return Err(err);
        }
    }
    let end_time = Instant::now();
    performance_metrics(
        start_time,
        end_time,
        "Time taken for MongoDB to insert depth history data : ",
    );

    Ok(true)
}

async fn insert_rune_pool_history_into_mongodb(
    mongodb: &MongoDB,
    resp: &RunePoolHistoryResponse,
) -> Result<bool> {
    let start_time = Instant::now();
    for rune_pool_history in &resp.intervals {
        if let Err(err) = mongodb.insert_rune_pool_history(rune_pool_history).await {
            dbg!("Failed to insert depth history data into database");
            return Err(err);
        }
    }
    let end_time = Instant::now();

    performance_metrics(
        start_time,
        end_time,
        "Time taken for MongoDB to insert rune pool history data : ",
    );

    Ok(true)
}

async fn insert_depth_history_into_postgres(
    postgres: &PostgreSQL,
    resp: &DepthHistoryResponse,
) -> Result<bool> {
    let start_time = Instant::now();

    for depth_history in &resp.intervals {
        if let Err(err) = postgres.insert_depth_history(depth_history).await {
            dbg!("Failed to insert depth history data into database");
            return Err(err);
        }
    }
    let end_time = Instant::now();

    performance_metrics(
        start_time,
        end_time,
        "Time taken for Postgres to insert depth history data : ",
    );

    Ok(true)
}

async fn insert_rune_pool_history_into_postgres(
    postgres: &PostgreSQL,
    resp: RunePoolHistoryResponse,
) -> Result<bool> {
    let start_time = Instant::now();

    for rune_pool_history in &resp.intervals {
        if let Err(err) = postgres.insert_rune_pool_history(rune_pool_history).await {
            dbg!("Failed to insert depth history data into database");
            return Err(err);
        }
    }
    let end_time = Instant::now();

    performance_metrics(
        start_time,
        end_time,
        "Time taken for Postgres to insert rune pool history data : ",
    );

    Ok(true)
}

async fn insert_depth_history_into_surrealdb(
    surrealdb: &SurrealDBWrapper,
    resp: &DepthHistoryResponse,
) -> Result<bool> {
    let start_time = Instant::now();

    for depth_history in &resp.intervals {
        if let Err(err) = surrealdb.insert_depth_history(depth_history).await {
            dbg!("Failed to insert depth history data into surrealDB");
            return Err(err);
        }
    }
    let end_time = Instant::now();

    performance_metrics(
        start_time,
        end_time,
        "Time taken for SurrealDB to insert depth history data : ",
    );

    Ok(true)
}

async fn insert_rune_pool_history_into_surrealdb(
    surrealdb: &SurrealDBWrapper,
    resp: &RunePoolHistoryResponse,
) -> Result<bool> {
    let start_time = Instant::now();

    for rune_pool_history in &resp.intervals {
        if let Err(err) = surrealdb.insert_rune_pool_history(rune_pool_history).await {
            dbg!("Failed to insert Rune pool history data into surrealDB");
            return Err(err);
        }
    }
    let end_time = Instant::now();

    performance_metrics(
        start_time,
        end_time,
        "Time taken for SurrealDB to insert rune pool history data : ",
    );

    Ok(true)
}

async fn insert_depth_history_into_leveldb(
    leveldb: &LevelDB,
    resp: &DepthHistoryResponse,
) -> Result<bool> {
    let start_time = Instant::now();

    for depth_history in &resp.intervals {
        if let Err(err) = leveldb.insert_depth_history(depth_history).await {
            dbg!("Failed to insert depth history data into levelDB");
            return Err(err);
        }
    }
    let end_time = Instant::now();

    performance_metrics(
        start_time,
        end_time,
        "Time taken for LevelDB to insert depth history data : ",
    );

    Ok(true)
}

async fn insert_rune_pool_history_into_leveldb(
    leveldb: &LevelDB,
    resp: &RunePoolHistoryResponse,
) -> Result<bool> {
    let start_time = Instant::now();

    for rune_pool_history in &resp.intervals {
        if let Err(err) = leveldb.insert_rune_pool_history(rune_pool_history).await {
            dbg!("Failed to insert Rune pool history data into LevelDB");
            return Err(err);
        }
    }
    let end_time = Instant::now();

    performance_metrics(
        start_time,
        end_time,
        "Time taken for LevelDB to insert rune pool history data : ",
    );

    Ok(true)
}
async fn insert_depth_history(
    mongodb: &MongoDB,
    postgres: &PostgreSQL,
    surrealdb: &SurrealDBWrapper,
    leveldb: &LevelDB,
) -> Result<bool> {
    let depth_history_url =
        "https://midgard.ninerealms.com/v2/history/depths/BTC.BTC?interval=hour&count=400";

    match reqwest::get(depth_history_url).await {
        Ok(response) => match response.json::<DepthHistoryResponse>().await {
            Ok(resp) => {
                // if let Err(err) = insert_depth_history_into_mongodb(mongodb, &resp).await {
                //     return Err(err);
                // }

                // if let Err(err) = insert_depth_history_into_postgres(postgres, &resp).await {
                //     return Err(err);
                // }

                // if let Err(err) = insert_depth_history_into_surrealdb(&surrealdb, &resp).await {
                //     return Err(err);
                // }

                if let Err(err) = insert_depth_history_into_leveldb(leveldb, &resp).await {
                    return Err(err);
                }
            }
            Err(err) => {
                dbg!(err);
                return Err(Error::OperationFailed(
                    "Cannot fetch data from api.".to_string(),
                ));
            }
        },
        Err(err) => {
            dbg!(err);
            return Err(Error::OperationFailed(
                "Cannot fetch data from api.".to_string(),
            ));
        }
    }

    Ok(true)
}

async fn insert_rune_pool_history(
    mongodb: &MongoDB,
    postgres: &PostgreSQL,
    surrealdb: &SurrealDBWrapper,
    leveldb: &LevelDB,
) -> Result<bool> {
    let rune_pool_history_url =
        format!("https://midgard.ninerealms.com/v2/history/runepool?interval=hour&count=400");

    match reqwest::get(&rune_pool_history_url).await {
        Ok(response) => match response.json::<RunePoolHistoryResponse>().await {
            Ok(resp) => {
                // if let Err(err) = insert_rune_pool_history_into_mongodb(mongodb, &resp).await {
                //     return Err(err);
                // }

                // if let Err(err) = insert_rune_pool_history_into_postgres(postgres, resp).await {
                //     return Err(err);
                // }

                // if let Err(err) = insert_rune_pool_history_into_surrealdb(&surrealdb, &resp).await {
                //     return Err(err);
                // }

                if let Err(err) = insert_rune_pool_history_into_leveldb(&leveldb, &resp).await {
                    return Err(err);
                }
            }
            Err(err) => {
                dbg!(err);
                return Err(Error::OperationFailed(
                    "Cannot fetch data from api.".to_string(),
                ));
            }
        },
        Err(err) => {
            dbg!(err);
            return Err(Error::OperationFailed(
                "Cannot fetch data from api.".to_string(),
            ));
        }
    }

    Ok(true)
}

pub async fn fetch_and_insert_data(
    Extension(database): Extension<Arc<DataBases>>,
) -> impl IntoResponse {
    let mongodb = &database.mongodb;
    let postgres = &database.postgres;
    let surrealdb = &database.surrealdb;
    let leveldb = &database.leveldb;

    if let Err(err) = insert_depth_history(mongodb, postgres, surrealdb, leveldb).await {
        dbg!(err);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to insert depth history data",
        )
            .into_response();
    }

    if let Err(err) = insert_rune_pool_history(mongodb, postgres, surrealdb, leveldb).await {
        dbg!(err);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to insert rune pool history data",
        )
            .into_response();
    }

    (StatusCode::OK, "Inserted data").into_response()
}

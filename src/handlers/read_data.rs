use std::{sync::Arc, time::Instant};

use axum::{response::IntoResponse, Extension};
use reqwest::StatusCode;

use crate::{
    db::{mongodb::MongoDB, postgres::PostgreSQL, DataBases},
    metrics::performance_metrics::performance_metrics,
    utils::types::Result,
};

async fn read_data_from_mongodb(mongodb: &MongoDB) -> Result<bool> {
    // reading depth history data...

    let start_time = Instant::now();

    mongodb.read_depth_history().await.unwrap();

    let end_time = Instant::now();

    performance_metrics(
        start_time,
        end_time,
        "Time taken for MongoDB to read depth history data : ",
    );

    // reading rune pool data...
    let start_time = Instant::now();

    mongodb.read_rune_pool_history().await.unwrap();

    let end_time = Instant::now();

    performance_metrics(
        start_time,
        end_time,
        "Time taken for MongoDB to read rune pool history data : ",
    );

    Ok(true)
}

pub async fn read_data_from_postgres(postgres: &PostgreSQL) -> Result<bool> {
    // reading depth history data...
    let start_time = Instant::now();

    postgres.read_depth_history().await.map_err(|e| e)?;

    let end_time = Instant::now();

    performance_metrics(
        start_time,
        end_time,
        "Time taken for Postgres to read depth history data : ",
    );

    // reading rune pool history data...
    let start_time = Instant::now();

    postgres.read_rune_pool_history().await.map_err(|e| e)?;

    let end_time = Instant::now();

    performance_metrics(
        start_time,
        end_time,
        "Time taken for Postgres to read rune pool history data : ",
    );

    Ok(true)
}

pub async fn read_data(Extension(database): Extension<Arc<DataBases>>) -> impl IntoResponse {
    let mongodb = &database.mongodb;
    let postgres = &database.postgres;

    let _ = read_data_from_mongodb(mongodb).await.map_err(|e| {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to read data from mongodb",
        )
            .into_response();
    });

    let _ = read_data_from_postgres(postgres).await.map_err(|e| {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to read data from postgres",
        )
            .into_response();
    });

    (StatusCode::OK, "Fetched data").into_response()
}

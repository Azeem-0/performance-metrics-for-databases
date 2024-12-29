use std::{sync::Arc, time::Instant};

use crate::{
    db::mongodb::MongoDB,
    metrics::insertion_time_metrics::log_insertion_metrics,
    models::{
        depth_history_model::DepthHistoryResponse, swaps_history_model::SwapsHistoryResponse,
    },
};
use axum::{response::IntoResponse, Extension};
use reqwest::StatusCode;

use super::mongodb_handler::insert_into_mongodb;
pub async fn fetch_and_insert_data(
    Extension(database): Extension<Arc<MongoDB>>,
) -> impl IntoResponse {
    let depth_history_url =
        format!("https://midgard.ninerealms.com/v2/history/depths/BTC.BTC?interval=hour&count=400");

    let mongodb = &*database;

    match reqwest::get(&depth_history_url).await {
        Ok(response) => match response.json::<DepthHistoryResponse>().await {
            Ok(resp) => {
                let start_time = Instant::now();
                // mongodb depth history - database insertion.
                for depth_history in resp.intervals {
                    if let Err(_) = mongodb.insert_depth_history(depth_history).await {
                        dbg!("Failed to insert depth history data into database");
                        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert data")
                            .into_response();
                    }
                }
                let end_time = Instant::now();

                log_insertion_metrics(start_time, end_time, "depth history");
            }
            Err(e) => {
                eprintln!("Failed to deserialize response: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse data").into_response();
            }
        },
        Err(e) => {
            eprintln!("Failed to fetch data: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch data").into_response();
        }
    }

    let swaps_history_url =
        format!("https://midgard.ninerealms.com/v2/history/swaps?interval=hour&count=400");

    match reqwest::get(&swaps_history_url).await {
        Ok(response) => match response.json::<SwapsHistoryResponse>().await {
            Ok(resp) => {
                let start_time = Instant::now();
                // mongodb swaps history - database insertion.
                for swaps_history in resp.intervals {
                    if let Err(_) = mongodb.insert_swaps_history(swaps_history).await {
                        dbg!("Failed to insert data into database");
                        return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to insert data")
                            .into_response();
                    }
                }
                let end_time = Instant::now();

                log_insertion_metrics(start_time, end_time, "swaps history");
            }
            Err(e) => {
                eprintln!("Failed to deserialize response: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to parse data").into_response();
            }
        },
        Err(e) => {
            eprintln!("Failed to fetch data: {:?}", e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch data").into_response();
        }
    }

    (StatusCode::OK, "Inserted data").into_response()
}

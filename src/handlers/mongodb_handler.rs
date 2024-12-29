use crate::{
    db::mongodb::MongoDB,
    models::depth_history_model::DepthHistoryResponse,
    utils::types::{Error, Result},
};

pub async fn insert_into_mongodb(mongodb: &MongoDB) -> Result<bool> {
    // Inserting depth history.

    let depth_history_url =
        format!("https://midgard.ninerealms.com/v2/history/depths/BTC.BTC?interval=hour&count=400");

    match reqwest::get(&depth_history_url).await {
        Ok(response) => match response.json::<DepthHistoryResponse>().await {
            Ok(resp) => {
                for depth_history in resp.intervals {
                    match mongodb.insert_depth_history(depth_history).await {
                        Ok(_) => (),
                        Err(_) => {
                            dbg!("Failed to insert depth history data into database");
                            return Err(Error::DataBaseInsertionFailed(
                                "Failed to insert into mongodb".to_string(),
                            ));
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to deserialize response: {:?}", e);
                return Err(Error::DataBaseError(
                    "Failed to  deserialize response:".to_string(),
                ));
            }
        },
        Err(e) => {
            eprintln!("Failed to fetch data: {:?}", e);
            return Err(Error::DataBaseError("Failed to  fetch data:".to_string()));
        }
    }

    // Inserting Swaps History.
    // let swaps_history_url =
    //     format!("https://midgard.ninerealms.com/v2/history/swaps?interval=hour&count=400");

    // match reqwest::get(&url).await {
    //     Ok(response) => match response.json::<SwapsHistoryResponse>().await {
    //         Ok(resp) => {
    //             for swaps_history in resp.intervals {
    //                 match db
    //                     .swaps_history_repo
    //                     .insert_swaps_history(&swaps_history)
    //                     .await
    //                 {
    //                     Ok(_) => (),
    //                     Err(_) => {
    //                         eprintln!("Failed to insert data into database");
    //                         return false;
    //                     }
    //                 }
    //             }
    //         }
    //         Err(e) => {
    //             eprintln!("Failed to deserialize response: {:?}", e);
    //             return false;
    //         }
    //     },
    //     Err(e) => {
    //         eprintln!("Failed to fetch data: {:?}", e);
    //         return false;
    //     }
    // }

    return Ok(true);
}

use std::collections::HashMap;
use std::error::Error;

use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use tokio::fs;

use crate::rs_err::RsErr;

pub async fn home() -> impl IntoResponse {
    match json("resources/home.json").await {
        Ok(content) => {
            let objs = serde_json::from_str(&content).unwrap();
            Json(objs)
        }
        Err(e) => {
            Json(
                serde_json::to_value(
                    RsErr {
                        status: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                        message: "Failed to fetch home (/) endpoints".to_owned(),
                        trace: e.to_string(),
                    }
                ).unwrap()
            )
        }
    }
}

async fn json(path: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(path).await.unwrap();
    let mut data: HashMap<String, String> = serde_json::from_str(&content)
        .unwrap();
    for (_, value) in &mut data {
        *value = value.replace("{port}", "3000");
    }
    let updated = serde_json::to_string(&data).unwrap();
    Ok(updated)
}

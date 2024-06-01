// The MIT License (MIT)
//
// Copyright (c) 2024 Aliaksei Bialiauski
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included
// in all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
use std::collections::HashMap;
use std::error::Error;

use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use tokio::fs;

use crate::rs_err::RsErr;

pub async fn home() -> impl IntoResponse {
    match json("resources/home.json").await {
        Ok(content) => {
            let objs = serde_json::from_str(&content).unwrap();
            Json(objs)
        }
        Err(e) => Json(
            serde_json::to_value(RsErr {
                status: StatusCode::INTERNAL_SERVER_ERROR.to_string(),
                message: "Failed to fetch home (/) endpoints".to_owned(),
                trace: e.to_string(),
            })
            .unwrap(),
        ),
    }
}

async fn json(path: &str) -> Result<String, Box<dyn Error>> {
    let content = fs::read_to_string(path).await.unwrap();
    let mut data: HashMap<String, String> = serde_json::from_str(&content).unwrap();
    for (_, value) in &mut data {
        *value = value.replace("{port}", "3000");
    }
    let updated = serde_json::to_string(&data).unwrap();
    Ok(updated)
}

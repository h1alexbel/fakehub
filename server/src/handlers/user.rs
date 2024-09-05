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

use crate::objects::json::json_user::JsonUser;
use crate::ServerConfig;
use axum::extract::{Path, State};
use axum::Json;
use serde_json::Value;
use std::collections::HashMap;

/// User by login.
pub async fn user(
    Path(params): Path<HashMap<String, String>>,
    State(config): State<ServerConfig>,
) -> Json<Value> {
    let login = params.get("login").expect("Failed to get login");
    let github = config.fakehub.main();
    let user = github.user(login).expect("Failed to get user");
    Json(JsonUser::new(user.clone()).as_json())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    #[test]
    fn returns_user() -> Result<()> {
        Ok(())
    }
}

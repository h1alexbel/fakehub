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
use serde_json::{Map, Value};
use std::collections::HashMap;

/// User by login.
// @todo #144:35min Test that 404 status code is thrown with "Not Found"
//  response. For now we only check JSON response with: message,
//  documentation_url, status nodes. Let's create unit test that will show that
//  404 is the status code of response.
pub async fn user(
    Path(params): Path<HashMap<String, String>>,
    State(config): State<ServerConfig>,
) -> Json<Value> {
    let login = params.get("login").expect("Failed to get login");
    let github = config.fakehub.main();
    match github.user(login) {
        Some(user) => Json(JsonUser::new(user.clone()).as_json()),
        None => {
            let mut response = Map::new();
            response.insert(
                String::from("message"),
                Value::String(String::from("Not Found")),
            );
            response.insert(
                String::from("documentation_url"),
                Value::String(String::from("https://github.com/h1alexbel/fakehub")),
            );
            response.insert(String::from("status"), Value::String(String::from("404")));
            Json(Value::Object(response))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::handlers::user::user;
    use crate::objects::fakehub::FakeHub;
    use crate::ServerConfig;
    use anyhow::Result;
    use axum::extract::{Path, State};
    use hamcrest::{equal_to, is, HamcrestMatcher};
    use std::collections::HashMap;

    #[tokio::test]
    async fn returns_user() -> Result<()> {
        let fakehub = FakeHub::default();
        let state = State(ServerConfig { fakehub });
        let mut params = HashMap::new();
        let expected = "jeff";
        params.insert(String::from("login"), String::from(expected));
        let user = user(Path(params), state).await;
        assert_that!(user["login"].as_str(), is(equal_to(Some(expected))));
        Ok(())
    }

    #[tokio::test]
    async fn returns_not_found() -> Result<()> {
        let fakehub = FakeHub::default();
        let state = State(ServerConfig { fakehub });
        let mut params = HashMap::new();
        params.insert(String::from("login"), String::from("unknown"));
        let response = user(Path(params), state).await;
        assert_that!(
            response["message"].as_str(),
            is(equal_to(Some("Not Found")))
        );
        assert_that!(
            response["documentation_url"].as_str(),
            is(equal_to(Some("https://github.com/h1alexbel/fakehub")))
        );
        assert_that!(response["status"].as_str(), is(equal_to(Some("404"))));
        Ok(())
    }
}

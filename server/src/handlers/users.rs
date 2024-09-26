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
use axum::extract::State;
use axum::Json;
use serde_json::Value;

/// All GitHub users, ordered by their id.
/// See: <a href="https://api.github.com/users">GitHub implementation</a>.
pub async fn users(State(config): State<ServerConfig>) -> Json<Vec<Value>> {
    let fakehub = config.fakehub;
    let github = fakehub.main();
    let mut users: Vec<Value> = github
        .lock()
        .expect("Failed to lock")
        .clone()
        .users()
        .iter()
        .map(|u| JsonUser::new(u.clone()).as_json())
        .collect();
    users.sort_by_key(|user| user.get("id").and_then(|id| id.as_u64()));
    Json(users)
}

#[cfg(test)]
mod tests {
    use crate::handlers::users::users;
    use crate::objects::fakehub::FakeHub;
    use crate::ServerConfig;
    use anyhow::Result;
    use axum::extract::State;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[tokio::test]
    #[allow(clippy::question_mark_used)]
    async fn returns_users() -> Result<()> {
        let users = users(State(ServerConfig {
            fakehub: FakeHub::default(),
        }))
        .await;
        let user = users.first().expect("Failed to get first JSON user");
        assert_that!(users.len(), is(equal_to(2)));
        assert_that!(user["id"].as_u64(), is(equal_to(Some(1))));
        assert_that!(user["login"].as_str(), is(equal_to(Some("jeff"))));
        Ok(())
    }
}

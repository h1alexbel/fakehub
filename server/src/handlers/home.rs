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
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use log::debug;

use crate::handlers::coordinates::Coordinates;
use crate::ServerConfig;

#[cfg(not(feature = "mirror_release"))]
pub use openapi;

#[cfg(feature = "mirror_release")]
pub use github_mirror as openapi;

use openapi::models::MetaRoot200Response;

/// Home handler.
pub async fn home(State(config): State<ServerConfig>) -> impl IntoResponse {
    let address = Coordinates::new(&config.fakehub).address();
    let response = Json(
        MetaRoot200Response::new(
            format!("{}/user", address),
            format!(
                "{}/settings/connections/applications{{/client_id}}",
                address
            ),
            format!("{}/authorizations", address),
            format!(
                "{}/search/code?q={{query}}{{&page,per_page,sort,order}}",
                address
            ),
            format!("{}/search/commits?q={{query}}{{&page,per_page,sort,order}}", address),
            format!("{}/user/emails", address),
            format!("{}/emojis", address),
            format!("{}/events", address),
            format!("{}/feeds", address),
            format!("{}/user/followers", address),
            format!("{}/user/following{{/target}}", address),
            format!("{}/gists{{/gist_id}}", address),
            format!("{}/search/issues?q={{query}}{{&page,per_page,sort,order}}", address),
            format!("{}/issues", address),
            format!("{}/user/keys", address),
            format!(
                "{}/search/labels?q={{query}}&repository_id={{repository_id}}{{&page,per_page}}",
                address
            ),
            format!("{}/notifications", address),
            format!("{}/orgs/{{org}}", address),
            format!(
                "{}/orgs/{{org}}/repos{{?type,page,per_page,sort}}",
                address
            ),
            format!("{}/orgs/{{org}}/teams", address),
            format!("{}/gists/public", address),
            format!("{}/rate_limit", address),
            format!("{}/repos/{{owner}}/{{repo}}", address),
            format!(
                "{}/search/repositories?q={{query}}{{&page,per_page,sort,order}}",
                address
            ),
            format!("{}/user/repos{{?type,page,per_page,sort}}", address),
            format!("{}/user/starred{{/owner}}{{/repo}}", address),
            format!("{}/gists/starred", address),
            format!("{}/users/{{user}}", address),
            format!("{}/user/orgs", address),
            format!(
                "{}/users/{{user}}/repos{{?type,page,per_page,sort}}",
                address
            ),
            format!(
                "{}/search/users?q={{query}}{{&page,per_page,sort,order}}",
                address
            ),
        )
    );
    debug!("{:?}", response);
    response
}

#[allow(clippy::question_mark_used)]
#[cfg(test)]
mod tests {
    use anyhow::Result;
    use axum::body::to_bytes;
    use axum::extract::State;
    use axum::response::IntoResponse;
    use hamcrest::{equal_to, is, HamcrestMatcher};
    use serde_json::{from_str, Value};
    use tokio::fs;

    use crate::handlers::home::home;
    use crate::objects::fakehub::FakeHub;
    use crate::ServerConfig;

    const BYTES_READ_LIMIT: usize = 10000;

    #[tokio::test]
    async fn returns_ok() -> Result<()> {
        assert_that!(
            IntoResponse::into_response(
                home(State(ServerConfig {
                    fakehub: FakeHub::with_addr(String::from("http://localhost:1234",)),
                }))
                .await
            )
            .status()
            .as_u16(),
            is(equal_to(200))
        );
        Ok(())
    }

    #[tokio::test]
    async fn returns_root_response() -> Result<()> {
        let actual: Value = from_str(&String::from_utf8(
            to_bytes(
                IntoResponse::into_response(
                    home(State(ServerConfig {
                        fakehub: FakeHub::with_addr(String::from("http://test:1234")),
                    }))
                    .await,
                )
                .into_body(),
                BYTES_READ_LIMIT,
            )
            .await?
            .to_vec(),
        )?)
        .expect("Failed to parse JSON");
        assert_that!(
            actual.to_string(),
            is(equal_to(
                fs::read_to_string("resources/home.json")
                    .await
                    .expect("Failed to read file")
            ))
        );
        Ok(())
    }
}

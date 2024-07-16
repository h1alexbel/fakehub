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

use crate::ServerConfig;
use openapi::models::MetaRoot200Response;

// @todo #9:25min Create unit tests for home.rs.
//  We need to create a few unit testes for home endpoints,
//  bootstrapping, and fetching. Don't forget to remove this puzzle.
pub async fn home(State(config): State<ServerConfig>) -> impl IntoResponse {
    let response = Json(
        MetaRoot200Response::new(
            format!("http://{}:{}/user", config.host, config.port),
            format!(
                "http://{}{}/settings/connections/applications{{/client_id}}",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/authorizations", config.host, config.port
            ),
            format!(
                "http://{}:{}/search/code?q={{query}}{{&page,per_page,sort,order}}",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/search/commits?q={{query}}{{&page,per_page,sort,order}}",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/user/emails",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/emojis",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/events",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/feeds",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/user/followers",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/user/following{{/target}}",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/gists{{/gist_id}}",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/search/issues?q={{query}}{{&page,per_page,sort,order}}",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/issues",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/user/keys",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/search/labels?q={{query}}&repository_id={{repository_id}}{{&page,per_page}}",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/notifications",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/orgs/{{org}}",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/orgs/{{org}}/repos{{?type,page,per_page,sort}}",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/orgs/{{org}}/teams",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/gists/public",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/rate_limit",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/repos/{{owner}}/{{repo}}",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/search/repositories?q={{query}}{{&page,per_page,sort,order}}",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/user/repos{{?type,page,per_page,sort}}",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/user/starred{{/owner}}{{/repo}}",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/gists/starred",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/users/{{user}}",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/user/orgs",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/users/{{user}}/repos{{?type,page,per_page,sort}}",
                config.host,
                config.port
            ),
            format!(
                "http://{}:{}/search/users?q={{query}}{{&page,per_page,sort,order}}",
                config.host,
                config.port
            ),
        )
    );
    debug!("{:?}", response);
    response
}

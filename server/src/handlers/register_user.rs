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
use crate::objects::user::User;
use crate::ServerConfig;
use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use log::info;

/// Register user.
///
/// # Fields
///
/// * `payload`: JSON payload
pub async fn register_user(
    State(config): State<ServerConfig>,
    Json(payload): Json<User>,
) -> Result<StatusCode, String> {
    let mut newcomer = User::new(payload.login.clone());
    let fakehub = config.fakehub;
    let github = fakehub.clone().main();
    match newcomer.register_in(&mut github.clone(), fakehub) {
        Ok(_) => {
            info!("New user is here. Hello @{}", newcomer.login);
            Ok(StatusCode::CREATED)
        }
        Err(e) => Err(format!("Can't register user @{}: {}", newcomer.login, e)),
    }
}

#[cfg(test)]
mod tests {
    use crate::handlers::register_user::register_user;
    use crate::objects::fakehub::FakeHub;
    use crate::objects::user::User;
    use crate::ServerConfig;
    use anyhow::Result;
    use axum::extract::State;
    use axum::Json;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[tokio::test]
    async fn registers_user() -> Result<()> {
        let server = ServerConfig {
            host: "0.0.0.0".into(),
            port: 1234,
            fakehub: FakeHub::default(),
        };
        let state = State(server);
        let status = register_user(state, Json::from(User::new(String::from("new1234"))))
            .await
            .expect("Failed to register user");
        assert_that!(status.as_u16(), is(equal_to(201)));
        Ok(())
    }

    #[should_panic(expected = "Failed to register user")]
    #[tokio::test]
    async fn panics_when_user_exists() {
        let server = ServerConfig {
            host: "0.0.0.0".into(),
            port: 1234,
            fakehub: FakeHub::default(),
        };
        let state = State(server);
        register_user(state, Json::from(User::new(String::from("jeff"))))
            .await
            .expect("Failed to register user");
    }
}

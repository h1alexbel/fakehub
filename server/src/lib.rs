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
use std::io;

use anyhow::Result;
use axum::routing::{get, post};
use axum::Router;
use tokio::net::TcpListener;

use crate::routes::home;
use crate::routes::register_user::register_user;
use crate::xml::storage::Storage;

mod objects;
pub mod report;
mod routes;
mod xml;
#[allow(unused_imports)]
#[macro_use]
extern crate hamcrest;

#[derive(Default)]
pub struct Server {
    port: usize,
}

impl Server {
    pub fn new(port: usize) -> Server {
        Server { port }
    }
}

#[derive(Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: usize,
}

// @todo #79:30min Log 404 NOT FOUND requests too.
//  Let's create a handler that would log requests failed with 404. Let's use
//  info!() for this one.
impl Server {
    pub async fn start(self) -> Result<()> {
        tracing_subscriber::fmt::init();
        Storage::new(Some("fakehub.xml"));
        let app = Router::new()
            .route("/", get(home::home))
            .route("/users", post(register_user))
            .with_state(ServerConfig {
                host: "0.0.0.0".into(),
                port: self.port,
            });
        let addr: String = format!("0.0.0.0:{}", self.port);
        let started: io::Result<TcpListener> = TcpListener::bind(addr.clone()).await;
        match started {
            Ok(listener) => axum::serve(listener, app).await?,
            Err(err) => {
                panic!("Can't bind address {}: '{}'", addr.clone(), err)
            }
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[test]
    fn creates_the_server() -> Result<()> {
        let server = crate::Server::new(1234);
        assert_that!(server.port, is(equal_to(1234)));
        Ok(())
    }
}

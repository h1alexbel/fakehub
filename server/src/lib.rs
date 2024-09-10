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
/*!
Fakehub server and storage.
 */
use std::io;

use anyhow::Result;
use axum::routing::{get, post};
use axum::Router;
use tokio::net::TcpListener;

use crate::handlers::home;
use crate::handlers::register_user::register_user;
use crate::handlers::user::user;
use crate::objects::fakehub::FakeHub;

/// Handlers.
pub mod handlers;
/// Fakehub objects.
pub mod objects;
/// Reports.
pub mod report;
#[allow(unused_imports)]
#[macro_use]
extern crate hamcrest;

/// Server.
#[derive(Default)]
pub struct Server {
    /// Port.
    port: usize,
}

impl Server {
    /// Create new server with port.
    ///
    /// * `port`: Server port
    ///
    /// returns: Server
    ///
    /// Examples:
    ///
    /// ```
    /// use server::Server;
    /// let server = Server::new(1234);
    /// ```
    pub fn new(port: usize) -> Server {
        Server { port }
    }
}

/// Server configuration.
#[derive(Clone)]
pub struct ServerConfig {
    /// Fakehub.
    pub fakehub: FakeHub,
}

// @todo #79:30min Log 404 NOT FOUND requests too.
//  Let's create a handler that would log requests failed with 404. Let's use
//  info!() for this one.
impl Server {
    /// Start a server.
    pub async fn start(self) -> Result<()> {
        let addr: String = format!("0.0.0.0:{}", self.port);
        let started: io::Result<TcpListener> = TcpListener::bind(addr.clone()).await;
        match started {
            Ok(listener) => axum::serve(
                listener,
                Router::new()
                    .route("/", get(home::home))
                    .route("/users", post(register_user))
                    .route("/users/:login", get(user))
                    .with_state(ServerConfig {
                        fakehub: FakeHub::with_addr(addr),
                    }),
            )
            .await
            .ok(),
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

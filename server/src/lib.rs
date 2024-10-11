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
use futures::future::BoxFuture;
use tokio::net::TcpListener;

use crate::handlers::home;
use crate::handlers::register_user::register_user;
use crate::handlers::user::user;
use crate::handlers::users::users;
use crate::objects::fakehub::FakeHub;
use crate::sys::sys_info::sys_info;

/// Handlers.
pub mod handlers;
/// Initialize.
pub mod init;
/// Fakehub objects.
pub mod objects;
/// Reports.
pub mod report;
/// System information.
pub mod sys;

#[allow(unused_imports)]
#[macro_use]
extern crate hamcrest;

/// Server.
pub trait Server {
    /// Start a server.
    #[allow(async_fn_in_trait)]
    fn start(&self) -> BoxFuture<'_, Result<()>>;
}

/// Default server.
#[derive(Default, Clone, Copy)]
pub struct DtServer {
    /// Port.
    port: usize,
}

impl DtServer {
    /// Create new server with port.
    ///
    /// * `port`: Server port
    ///
    /// returns: Server
    ///
    /// Examples:
    ///
    /// ```
    /// use fakehub_server::DtServer;
    /// let server = DtServer::new(1234);
    /// ```
    pub fn new(port: usize) -> DtServer {
        DtServer { port }
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
impl Server for DtServer {
    /// Start a server.
    fn start(&self) -> BoxFuture<'_, Result<()>> {
        Box::pin(async move {
            let addr: String = format!("0.0.0.0:{}", self.port);
            sys_info();
            let started: io::Result<TcpListener> = TcpListener::bind(addr.clone()).await;
            match started {
                Ok(listener) => axum::serve(
                    listener,
                    Router::new()
                        .route("/", get(home::home))
                        .route("/users", post(register_user))
                        .route("/users/:login", get(user))
                        .route("/users", get(users))
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
        })
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[test]
    fn creates_the_server() -> Result<()> {
        let server = crate::DtServer::new(1234);
        assert_that!(server.port, is(equal_to(1234)));
        Ok(())
    }
}

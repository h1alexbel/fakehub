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
use axum::routing::get;
use axum::Router;
use tokio::net::TcpListener;

use crate::routes::home;
use crate::xml::storage::touch_storage;

mod routes;
mod xml;

#[derive(Default)]
pub struct Server {
    port: usize,
}

impl Server {
    pub fn new(port: usize) -> Server {
        Server { port }
    }
}

impl Server {
    pub async fn start(self) -> anyhow::Result<()> {
        touch_storage(Some("fakehub.xml"));
        let app: Router = Router::new().route("/", get(home::home));
        let addr: String = format!("0.0.0.0:{}", self.port);
        let started: std::io::Result<TcpListener> = TcpListener::bind(addr.clone()).await;
        match started {
            Ok(listener) => axum::serve(listener, app).await?,
            Err(err) => {
                panic!("Can't bind address {}: '{}'", addr.clone(), err)
            }
        };
        Ok(())
    }
}

mod tests {

    #[test]
    fn creates_the_server() -> anyhow::Result<()> {
        let server = crate::Server::new(1234);
        assert_eq!(server.port, 1234);
        Ok(())
    }
}

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
use clap::Parser;
use log::info;

use server::Server;

use crate::args::{Args, Command};

mod args;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.command {
        Command::Start(start_args) => {
            if start_args.verbose {
                tracing_subscriber::fmt()
                    .with_max_level(tracing::Level::DEBUG)
                    .init()
            } else {
                tracing_subscriber::fmt::init();
            }
            info!("Starting server on port {}", start_args.port);
            let server = Server::new(start_args.port);
            match server.start().await {
                Ok(_) => info!("Server started successfully on port {}", start_args.port),
                Err(e) => panic!(
                    "{}",
                    format!("Failed to start server on port {}: {}", start_args.port, e)
                ),
            }
        }
    }
}

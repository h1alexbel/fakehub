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
Fakehub cli.
 */
use clap::Parser;
use log::{error, info};

/// System calls.
pub mod sys;

use fakehub_server::{DtServer, Server};

use crate::args::{Args, Command};
use crate::sys::current_port::current_port;
use crate::sys::kill_unix::kill_unix;
use fakehub_server::init::server_with_init_state::ServerWithInitState;
use fakehub_server::sys::instance_os::instance_os;
use fakehub_server::sys::sys_info::sys_info;
#[cfg(target_os = "windows")]
use std::os::windows::process::CommandExt;

mod args;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.command {
        Command::Start(start) => {
            if start.verbose {
                tracing_subscriber::fmt()
                    .with_max_level(tracing::Level::DEBUG)
                    .init()
            } else {
                tracing_subscriber::fmt::init();
            }
            info!("Starting server on port {}", start.port);
            if start.detach {
                match std::env::current_exe() {
                    Ok(buf) => {
                        let mut command = std::process::Command::new(buf);
                        command
                            .arg("start")
                            .arg("--port")
                            .arg(start.port.to_string())
                            .stdout(std::process::Stdio::null())
                            .stderr(std::process::Stdio::null())
                            .stdin(std::process::Stdio::null());
                        if start.verbose {
                            command.arg("--verbose");
                        }
                        #[cfg(target_os = "windows")]
                        // Detached windows process flag.
                        command.creation_flags(0x00000008);
                        match command.spawn() {
                            Ok(_) => info!(
                                "Server is running in detached mode on port {}",
                                start.port
                            ),
                            Err(err) => {
                                error!("Failed to spawn detached process: {}", err)
                            }
                        }
                        sys_info();
                        return;
                    }
                    Err(err) => {
                        panic!("Failed to start fakehub server in detached mode: {}", err)
                    }
                };
            }
            let base = DtServer::new(start.port);
            let server: Box<dyn Server> = if !start.init.is_empty() {
                Box::new(ServerWithInitState::new(base, start.init))
            } else {
                Box::new(base)
            };
            match server.start().await {
                Ok(_) => info!("Server started successfully on port {}", start.port),
                Err(e) => panic!(
                    "{}",
                    format!("Failed to start server on port {}: {}", start.port, e)
                ),
            }
        }
        // @todo #77:45min Implement kill_windows_port(id) for windows OS.
        //  Currently, we don't support port killing on Windows. Check
        //  <a href="https://github.com/h1alexbel/fakehub/pull/159">this</a> pull
        //  request for more information.
        Command::Stop => {
            tracing_subscriber::fmt::init();
            info!("Stopping fakehub...");
            let port = current_port();
            let result = match instance_os().as_str() {
                "linux" | "macos" => kill_unix(port),
                _ => {
                    error!(
                        "Cannot stop server on port {}, since we probably don't support {} platform",
                        port, instance_os()
                    );
                    false
                }
            };
            if result {
                info!("fakehub stopped");
            } else {
                error!("Cannot stop fakehub on port: {}", port);
            }
        }
    }
}

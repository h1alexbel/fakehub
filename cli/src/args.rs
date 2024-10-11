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
use clap::{ArgAction, Parser, Subcommand};

// @todo #41:15min Add --report argument.
//  Let's add --report option for generating reports in desired formats:
//  We should support following formats: xml, tex, and txt. User should have
//  ability to generate report in multiple formats as well: --report tex,xml,txt.
#[derive(Parser, Debug)]
#[command(name = "fakehub", version = env!("CARGO_PKG_VERSION"))]
pub(crate) struct Args {
    #[arg(short = 'v', action = ArgAction::Version)]
    v: Option<bool>,
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Command {
    /// Start the server
    #[command(about = "Start the server")]
    Start(StartArgs),
    #[command(about = "Stop the server")]
    Stop,
}

#[derive(Parser, Debug)]
pub(crate) struct StartArgs {
    /// The port to run
    #[arg(short, long, default_value_t = 3000)]
    pub(crate) port: usize,
    /// Verbose output.
    #[arg(short, long, default_value = "false")]
    pub(crate) verbose: bool,
    /// Run in detach mode.
    #[arg(short, long, default_value = "false")]
    pub(crate) detach: bool,
    /// Path to file or directory with initial state.
    #[arg(long, default_value = "")]
    pub(crate) init: String,
}

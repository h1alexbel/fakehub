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
use std::str;

use anyhow::Result;
use assert_cmd::Command;

#[test]
fn outputs_help() -> Result<()> {
    let assertion = Command::cargo_bin("cli")?.arg("--help").assert();
    let bytes = assertion.get_output().stdout.as_slice();
    let output = str::from_utf8(bytes)?;
    assert!(output.contains("help"));
    assert!(output.contains("start"));
    assert!(output.contains("Start the server"));
    assert!(output.contains("--help"));
    assert!(output.contains("Print help"));
    Ok(())
}

#[test]
fn outputs_start_opts() -> Result<()> {
    let assertion = Command::cargo_bin("cli")?
        .arg("start")
        .arg("--help")
        .assert();
    let bytes = assertion.get_output().stdout.as_slice();
    let output = str::from_utf8(bytes)?;
    assert!(output.contains("--port"));
    assert!(output.contains("The port to run [default: 3000]"));
    Ok(())
}

// @todo #43:30min Enable starts_server integration test.
//  We should enable this integration test that checks whether server starts or
//  not. This test should output success message in info!. For now it does not
//  work. Probably some error with logging configuration. Don't forget to
//  remove this puzzle.
#[test]
#[ignore]
fn starts_server() -> Result<()> {
    let assertion = Command::cargo_bin("cli")?
        .arg("start")
        .arg("--port 8080")
        .assert();
    let bytes = assertion.get_output().stdout.as_slice();
    let output = str::from_utf8(bytes)?;
    assert!(output.contains("Server started successfully on port 8080"));
    Ok(())
}

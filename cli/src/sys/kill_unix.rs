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

use log::{error, info};

/// Kill UNIX port.
// @todo #77:60min Investigate why lsof does not work in tests, while
//  killport tool kills port properly. Due to this problem, we can't create
//  integration test case with fakehub start -d -> fakehub stop. Don't forget
//  to remove this puzzle.
pub fn kill_unix(port: usize) -> bool {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg(format!("lsof -ti :{} | xargs kill", port))
        .output()
        .expect("failed to kill process");
    let result;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.is_empty() {
            error!("No process found running on port {}: unable to kill", port);
            result = false;
        } else {
            error!("Failed to kill process on port {}: {}", port, stderr);
            result = false;
        }
    } else {
        result = true;
        info!("Port {} killed", port);
    }
    result
}

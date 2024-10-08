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

/// Kill UNIX port.
pub fn kill_unix(port: usize) {
    std::process::Command::new("sh")
        .arg("-c")
        // .arg(format!("killport {}", port))
        .arg(format!("lsof -ti :{} | xargs kill", port))
        .output()
        .unwrap_or_else(|_| panic!("failed to kill process on port {}", port));
}

#[cfg(test)]
mod tests {
    use crate::sys::kill_unix::kill_unix;
    use anyhow::Result;
    use defer::defer;
    use tokio::task;
    use crate::Server;

    #[tokio::test]
    #[cfg(not(target_os = "windows"))]
    async fn kills_unix() -> Result<()> {
        let port = 3000;
        let _defer = defer(|| kill_unix(port));
        let server = task::spawn(async move {
            Server::new(port)
                .start()
                .await
                .expect("failed to start server");
        });
        // check that its removed
        Ok(())
    }
}

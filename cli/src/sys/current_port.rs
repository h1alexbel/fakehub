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

/// Current fakehub port.
pub fn current_port() -> usize {
    let output = std::process::Command::new("sh")
        .arg("-c")
        .arg("lsof -i -P -n | grep fakehub | awk '{print $9}' | cut -d ':' -f2")
        .output()
        .expect("failed to get fakehub current port");
    let port = String::from_utf8(output.stdout).expect("failed to parse stdout");
    let trimmed = port.trim();
    trimmed
        .parse::<usize>()
        .expect("failed to convert port to usize")
}

#[cfg(test)]
mod tests {
    #[cfg_attr(target_os = "windows", allow(unused_imports))]
    use crate::sys::current_port::current_port;
    #[cfg_attr(target_os = "windows", allow(unused_imports))]
    use anyhow::Result;
    #[cfg_attr(target_os = "windows", allow(unused_imports))]
    use assert_cmd::Command;
    #[cfg_attr(target_os = "windows", allow(unused_imports))]
    use defer::defer;

    #[cfg(not(target_os = "windows"))]
    #[test]
    #[allow(clippy::question_mark_used)]
    fn returns_current_port_from_lsof() -> Result<()> {
        let port = 3000;
        let _defer = defer(|| kill(3000));
        Command::cargo_bin("fakehub")?
            .arg("start")
            .arg("-d")
            .assert();
        let output = std::process::Command::new("sh")
            .arg("-c")
            .arg("lsof -i -P -n | grep fakehub")
            .output()
            .expect("failed to get fakehub current port");
        print!("{:?}", output);
        assert_eq!(current_port(), port);
        Ok(())
    }

    #[cfg_attr(target_os = "windows", allow(dead_code))]
    fn kill(port: usize) {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("killport {}", port))
            .output()
            .unwrap_or_else(|_| panic!("Failed to kill process on port {}", port));
    }
}

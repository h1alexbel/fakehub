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
#[allow(clippy::question_mark_used)]
#[cfg(test)]
mod tests {
    use anyhow::Result;
    use assert_cmd::Command;
    use std::str;
    use std::time::Duration;

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
        assert!(output.contains("-p"));
        assert!(output.contains("--port"));
        assert!(output.contains("The port to run [default: 3000]"));
        assert!(output.contains("-v"));
        assert!(output.contains("--verbose"));
        assert!(output.contains("Verbose output"));
        assert!(output.contains("-d"));
        assert!(output.contains("--detach"));
        assert!(output.contains("Run in detach mode"));
        Ok(())
    }

    // @todo #129:35min Find a way to run slow tests separately from fast tests.
    //  This test `accepts_request_in_detached_mode` runs a way longer than
    //  other unit tests. Let's mark such long tests as slow and run them
    //  separately from fast test. Locally, developers will run only fast
    //  tests, while CI server will run both. Check
    //  <a href="https://www.yegor256.com/2023/08/22/fast-vs-deep-testing.html">this link<a>
    //  for more information about this idea.
    #[tokio::test]
    #[cfg(not(target_os = "windows"))]
    // @todo #129:60min Create similar integration test for windows platform.
    //  Now we have test for linux and macos. However, we need to maintain
    //  similar test case for windows as well.
    async fn accepts_request_in_detached_mode() -> Result<()> {
        let assertion = Command::cargo_bin("cli")?.arg("start").arg("-d").assert();
        let bytes = assertion.get_output().stdout.as_slice();
        let output = str::from_utf8(bytes)?;
        assert!(
            output.contains("Server is running in detached mode on port 3000"),
            "Output should contain logs that server started in detached mode"
        );
        let request = reqwest::Client::new();
        let mut retries = 10;
        let mut status = None;
        while retries > 0 {
            tokio::time::sleep(Duration::from_secs(1)).await;
            match request.get("http://localhost:3000").send().await {
                Ok(home) => {
                    status = Some(home.status());
                    break;
                }
                Err(_) => {
                    retries -= 1;
                }
            }
        }
        assert_eq!(status.expect("Failed to retrieve status"), 200);
        kill(3000);
        Ok(())
    }

    fn kill(port: usize) {
        std::process::Command::new("sh")
            .arg("-c")
            .arg(format!("killport {}", port))
            .output()
            .unwrap_or_else(|_| panic!("Failed to kill process on port {}", port));
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

    // @todo #82:30min Enable runs_in_verbose_mode test.
    //  We should enable this test right after we find out how to shutdown the server
    //  in test. This problem is similar to
    //  https://github.com/h1alexbel/fakehub/issues/76.
    #[test]
    #[ignore]
    fn runs_in_verbose_mode() -> Result<()> {
        let assertion = Command::cargo_bin("cli")?
            .arg("start")
            .arg("--verbose")
            .assert();
        let bytes = assertion.get_output().stdout.as_slice();
        let output = str::from_utf8(bytes)?;
        assert!(output.contains("DEBUG"));
        Ok(())
    }
}

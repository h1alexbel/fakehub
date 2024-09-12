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
    use std::time::Duration;
    use std::{str, thread};

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
        assert!(output.contains("--verbose"));
        assert!(output.contains("-v"));
        assert!(output.contains("Verbose output"));
        Ok(())
    }

    #[test]
    fn runs_in_detached_mode() -> Result<()> {
        let mut command = std::process::Command::new("cli");
        command.arg("start").arg("-d");
        let child = command
            .spawn()
            .expect("Failed to start the detached process");
        thread::sleep(Duration::from_secs(1));
        assert!(child.id() > 0, "Detached process did not start correctly.");
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

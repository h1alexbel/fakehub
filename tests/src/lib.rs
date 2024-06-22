#[cfg(test)]
mod tests {
    use std::str;

    use anyhow::Result;
    use assert_cmd::Command;

    #[test]
    fn should_output_help() -> Result<()> {
        let assertion = Command::cargo_bin("cli")?.arg("--help").assert();
        let bytes = assertion.get_output().stdout.as_slice();
        let output = str::from_utf8(bytes)?;
        assert!(output.contains("The port to run"));
        assert!(output.contains("3000"));
        Ok(())
    }
}

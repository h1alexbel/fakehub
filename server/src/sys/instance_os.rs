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
/// Instance OS.
pub fn instance_os() -> String {
    String::from(std::env::consts::OS)
}

#[cfg(test)]
mod tests {

    use crate::sys::instance_os::instance_os;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[cfg(target_os = "linux")]
    #[test]
    fn returns_os_on_linux() -> Result<()> {
        assert_that!(&instance_os(), is(equal_to("linux")));
        Ok(())
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn returns_os_on_mac() -> Result<()> {
        assert_that!(&instance_os(), is(equal_to("macos")));
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn returns_os_on_windows() -> Result<()> {
        assert_that!(&instance_os(), is(equal_to("windows")));
        Ok(())
    }
}

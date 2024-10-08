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
use crate::sys::instance_os::instance_os;
use log::info;

/// System information.
pub fn sys_info() {
    info!("OS: {}", instance_os());
    info!("PID: {}", std::process::id());
}

#[cfg(test)]
mod tests {
    use crate::sys::sys_info::sys_info;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};
    use log::Level::Info;

    #[cfg(target_os = "macos")]
    #[test]
    fn logs_system_info_on_mac() -> Result<()> {
        testing_logger::setup();
        sys_info();
        testing_logger::validate(|logs| {
            assert_that!(logs.len(), is(equal_to(2)));
            assert_that!(&logs[0].body, is(equal_to("OS: macos")));
            assert_that!(logs[0].level, is(equal_to(Info)));
            assert_that!(logs[1].level, is(equal_to(Info)));
        });
        Ok(())
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn logs_system_info_on_linux() -> Result<()> {
        testing_logger::setup();
        sys_info();
        testing_logger::validate(|logs| {
            assert_that!(logs.len(), is(equal_to(2)));
            assert_that!(&logs[0].body, is(equal_to("OS: linux")));
            assert_that!(logs[0].level, is(equal_to(Info)));
            assert_that!(logs[1].level, is(equal_to(Info)));
        });
        Ok(())
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn logs_system_info_windows() -> Result<()> {
        testing_logger::setup();
        sys_info();
        testing_logger::validate(|logs| {
            assert_that!(logs.len(), is(equal_to(2)));
            assert_that!(&logs[0].body, is(equal_to("OS: windows")));
            assert_that!(logs[0].level, is(equal_to(Info)));
            assert_that!(logs[1].level, is(equal_to(Info)));
        });
        Ok(())
    }
}

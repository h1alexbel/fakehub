use std::fs::File;
use std::io::Write;

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
use anyhow::Result;
use log::info;

#[derive(Default)]
#[allow(dead_code)]
pub struct Storage {
    pub(crate) path: String,
}

const INIT_XML: &str = "<root>\
                        <github><users/></github>\
                        </root>";

impl Storage {
    pub fn new(path: Option<&str>) -> Storage {
        let location = path.unwrap_or("fakehub.xml");
        info!("Initializing XML storage: {location}");
        let mut file = match File::create(location) {
            Ok(file) => file,
            Err(err) => {
                panic!("fakehub storage failed to initialize in '{location}': {err}");
            }
        };
        if let Err(err) = file.write_all(INIT_XML.as_bytes()) {
            panic!("Failed to write initial content to '{}': {}", location, err);
        }
        info!("'{}' initialized", location);
        Storage {
            path: String::from(location),
        }
    }
}

// @todo #17:35min Implement #xml function in Storage.
//  This function should return full XML storage has at the moment. #xml
//  function should be thread-safe, as it intended to be used concurrently.
//  Don't forget to create a unit tests related to #xml function.
impl Storage {
    #[allow(dead_code)]
    pub fn xml() -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};
    use tempdir::TempDir;

    use crate::xml::storage::Storage;

    #[test]
    fn creates_xml_storage() -> Result<()> {
        let temp = TempDir::new("temp")?;
        let path = temp.path().join("fakehub.xml");
        let storage = path.to_str();
        Storage::new(storage);
        assert_that!(path.exists(), is(equal_to(true)));
        Ok(())
    }

    #[test]
    fn reads_initial_content() -> Result<()> {
        let temp = TempDir::new("temp")?;
        let path = temp.path().join("fakehub.xml");
        Storage::new(path.to_str());
        let xml = fs::read_to_string(path).unwrap();
        let expected = "<root><github><users/></github></root>";
        assert_that!(xml, is(equal_to(String::from(expected))));
        Ok(())
    }

    #[test]
    fn creates_xml_storage_with_different_name() -> Result<()> {
        let temp = TempDir::new("temp")?;
        let path = temp.path().join("test.xml");
        let storage = path.to_str();
        Storage::new(storage);
        assert_that!(path.exists(), is(equal_to(true)));
        Ok(())
    }
}

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
use log::info;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Default, Debug, Serialize, Deserialize)]
#[allow(dead_code)]
/// Storage.
pub struct Storage {
    pub(crate) path: String,
}

const INIT_XML: &str = "<root>
<github><users/></github>
</root>
";

impl Storage {
    /// New storage.
    ///
    /// # Fields
    /// * `path`: Storage path
    ///
    /// # Examples
    ///
    /// ```
    /// use server::xml::storage::Storage;
    /// let storage = Storage::new(Some("test.xml"));
    /// ```
    /// Or use it with default path:
    /// ```
    /// use server::xml::storage::Storage;
    /// let storage = Storage::new(None);
    /// ```
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

    /// Returns full XML from the storage.
    // @todo #75:60min Make xml() thread-safe.
    //  We should make this function thread-safe in order to get sequential of
    //  reads and write to the store. Don't forget to create a unit-test that
    //  checks concurrency cases.
    pub fn xml(self) -> String {
        let mut file = File::open(self.path).expect("Can't open file");
        let mut xml = String::new();
        file.read_to_string(&mut xml)
            .expect("Can't read file with XML");
        xml
    }
}

#[allow(clippy::question_mark_used)]
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
        let xml = fs::read_to_string(path)?;
        let expected = "<root>\n<github><users/></github>\n</root>\n";
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

    #[test]
    fn outputs_full_xml() -> Result<()> {
        let temp = TempDir::new("temp")?;
        let path = temp.path().join("test.xml");
        let xml = Storage::new(path.to_str()).xml();
        assert_that!(xml.contains("<root>"), is(equal_to(true)));
        assert_that!(xml.contains("<github>"), is(equal_to(true)));
        assert_that!(xml.contains("<users/>"), is(equal_to(true)));
        Ok(())
    }
}

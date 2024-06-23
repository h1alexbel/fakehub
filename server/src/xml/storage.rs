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
use std::fs::File;

use log::info;

pub fn touch_storage(path: Option<&str>) -> File {
    let location = path.unwrap_or("fakehub.xml");
    info!("Initializing XML storage: {location}");
    match File::create(location) {
        Ok(file) => {
            info!("'{location}' initialized");
            file
        }
        Err(err) => {
            panic!("fakehub storage failed to initialize in '{location}': {err}")
        }
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use tempdir::TempDir;

    use crate::xml::storage::touch_storage;

    #[test]
    fn creates_xml_storage() -> Result<()> {
        let temp = TempDir::new("temp")?;
        let path = temp.path().join("fakehub.xml");
        let storage = path.to_str();
        touch_storage(storage);
        assert!(
            path.exists(),
            "storage file {:?} was not created, but should be",
            storage
        );
        Ok(())
    }

    #[test]
    fn creates_xml_storage_with_different_name() -> Result<()> {
        let temp = TempDir::new("temp")?;
        let path = temp.path().join("tests.xml");
        let storage = path.to_str();
        touch_storage(storage);
        assert!(
            path.exists(),
            "storage file {:?} was not created, but should be",
            storage
        );
        Ok(())
    }
}

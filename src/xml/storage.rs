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
use std::fmt::Error;
use std::fs::File;

use log::info;

pub fn touch_storage(name: Option<&str>) -> Result<File, Error> {
    let path = name.unwrap_or("fakehub.xml");
    info!("Initializing XML storage: {path}");
    let creating = File::create(path);
    match creating {
        Ok(file) => {
            info!("'{path}' initialized");
            Ok(file)
        }
        Err(err) => {
            panic!("fakehub storage failed to initialize in '{path}': {err}")
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::xml::storage::touch_storage;

    #[test]
    fn creates_xml_storage() -> anyhow::Result<()> {
        touch_storage(None).unwrap();
        let storage = "fakehub.xml";
        let exists = Path::new(storage).exists();
        assert!(
            exists,
            "storage file '{storage}' was not created, but should be"
        );
        Ok(())
    }

    #[test]
    fn creates_xml_storage_with_different_name() -> anyhow::Result<()> {
        let path = "test.xml";
        touch_storage(Some(path)).unwrap();
        let exists = Path::new(path).exists();
        assert!(
            exists,
            "storage file '{path}' was not created, but should be"
        );
        Ok(())
    }
}

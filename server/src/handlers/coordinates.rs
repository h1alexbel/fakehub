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
use crate::objects::fakehub::FakeHub;
use regex::Regex;

/// Coordinates.
#[derive(Clone)]
pub struct Coordinates<'a> {
    fakehub: &'a FakeHub,
    regex: Regex,
}

impl<'a> Coordinates<'a> {
    /// New.
    pub fn new(fakehub: &'a FakeHub) -> Coordinates<'a> {
        Coordinates {
            fakehub,
            regex: Regex::new(r"([^;]+);node:([a-f0-9]+)")
                .expect("Failed to create regex"),
        }
    }

    /// Address.
    pub fn address(&self) -> String {
        let coords = self.fakehub.coords();
        let captures = self
            .regex
            .captures(coords.as_str())
            .expect("Failed to get captures");
        String::from(captures.get(1).map_or("", |m| m.as_str()))
    }

    /// Node ID.
    pub fn node_id(&self) -> String {
        let coords = self.fakehub.coords();
        let captures = self
            .regex
            .captures(coords.as_str())
            .expect("Failed to get captures");
        String::from(captures.get(2).map_or("", |m| m.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use crate::handlers::coordinates::Coordinates;
    use crate::objects::fakehub::FakeHub;
    use anyhow::Result;
    use chrono::{TimeZone, Utc};
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[test]
    fn returns_default_address() -> Result<()> {
        let fakehub = FakeHub::new(Utc.with_ymd_and_hms(2024, 9, 1, 9, 10, 11).unwrap());
        let coordinates = Coordinates::new(&fakehub);
        let address = coordinates.address();
        assert_that!(address, is(equal_to(String::from("localhost"))));
        Ok(())
    }

    #[test]
    fn returns_bind_address() -> Result<()> {
        let expected = String::from("localhost:1234");
        let fakehub = FakeHub::with_addr(expected.clone());
        let coordinates = Coordinates::new(&fakehub);
        let address = coordinates.address();
        assert_that!(address, is(equal_to(expected)));
        Ok(())
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn returns_node_id() -> Result<()> {
        let fakehub = FakeHub::new(Utc.with_ymd_and_hms(2024, 9, 1, 9, 10, 11).unwrap());
        let coordinates = Coordinates::new(&fakehub);
        let id = coordinates.node_id();
        assert_that!(
            id,
            is(equal_to(String::from("305be946d516494d20c7c10f6d0020f9")))
        );
        Ok(())
    }
}

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
use crate::objects::github::GitHub;
use uuid::Uuid;

/// Fakehub.
pub struct Fakehub {
    hubs: Vec<GitHub>,
}

impl Default for Fakehub {
    fn default() -> Fakehub {
        Fakehub {
            hubs: vec![GitHub {
                id: Uuid::new_v4(),
                url: String::from("https://github.com"),
            }],
        }
    }
}

impl Fakehub {
    /// New fakehub
    pub fn new(hubs: Vec<GitHub>) -> Fakehub {
        Fakehub { hubs }
    }

    /// Add new GitHub
    pub fn add(mut self, github: GitHub) {
        self.hubs.push(github);
    }
}

#[cfg(test)]
mod tests {

    use crate::objects::fakehub::Fakehub;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[test]
    fn creates_default_fakehub() -> Result<()> {
        let fakehub = Fakehub::default();
        let default = fakehub.hubs.first().expect("Can not obtain GitHub");
        assert_that!(default.id.is_nil(), is(equal_to(false)));
        assert_that!(&default.url, is(equal_to("https://github.com")));
        Ok(())
    }
}

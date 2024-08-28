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
use crate::objects::inversed::inversed;
use std::collections::HashMap;
use uuid::Uuid;

/// Fakehub.
#[derive(Clone)]
pub struct Fakehub {
    hubs: HashMap<Uuid, String>,
    inverse: HashMap<String, Uuid>,
}

impl Default for Fakehub {
    fn default() -> Fakehub {
        let mut hubs: HashMap<Uuid, String> = HashMap::new();
        hubs.insert(Uuid::new_v4(), String::from("https://github.com"));
        Fakehub {
            hubs: hubs.clone(),
            inverse: inversed(hubs),
        }
    }
}

impl Fakehub {
    /// Add new GitHub
    pub fn add(&mut self, github: GitHub) {
        self.hubs.insert(github.id, github.url.clone());
        self.inverse.insert(github.url, github.id);
    }

    /// All GitHub instances.
    pub fn githubs(self) -> Vec<GitHub> {
        self.hubs
            .iter()
            .map(|(k, v)| GitHub {
                id: *k,
                url: v.clone(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::fakehub::Fakehub;
    use crate::objects::github::GitHub;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};
    use uuid::Uuid;

    #[test]
    fn creates_default_fakehub() -> Result<()> {
        let fakehub = Fakehub::default();
        let default = fakehub
            .inverse
            .get("https://github.com")
            .expect("Failed to get GitHub");
        assert_that!(default.is_nil(), is(equal_to(false)));
        Ok(())
    }

    #[test]
    fn adds_github() -> Result<()> {
        let mut fakehub = Fakehub::default();
        let url = String::from("https://jeff.github.com");
        let expected = Uuid::new_v4();
        fakehub.add(GitHub {
            id: expected,
            url: url.clone(),
        });
        let id = fakehub.inverse.get(&url).expect("Failed to get GitHub");
        assert_that!(*id, is(equal_to(expected)));
        Ok(())
    }

    #[test]
    fn returns_all_github_instances_after_add() -> Result<()> {
        let mut fakehub = Fakehub::default();
        fakehub.add(GitHub {
            id: Uuid::new_v4(),
            url: String::from("https://test.github.com"),
        });
        let instances = fakehub.githubs();
        assert_that!(instances.len(), is(equal_to(2)));
        Ok(())
    }
}

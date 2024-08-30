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
use crate::objects::user::User;
use std::collections::HashMap;
use uuid::Uuid;

/// Fakehub.
/// ```
/// use hamcrest::assert_that;
/// use hamcrest::{equal_to, is, HamcrestMatcher};
/// use server::objects::fakehub::Fakehub;
///
/// let fakehub = Fakehub::default();
/// let github = fakehub.urled.get("https://github.com").expect("Failed to get GitHub");
/// let jeff = github.user("jeff");
/// assert_that!(&jeff.username, is(equal_to("jeff")));
/// ```
/// To add new GitHub:
/// ```
/// use hamcrest::assert_that;
/// use hamcrest::{equal_to, is, HamcrestMatcher};
/// use std::collections::HashMap;
/// use uuid::Uuid;
/// use server::objects::fakehub::Fakehub;
/// use server::objects::github::GitHub;
///
///
/// let mut fakehub = Fakehub::default();
/// let custom = String::from("https://jeff.github.com");
/// let expected = Uuid::new_v4();
/// fakehub.add(GitHub {id: expected, url: custom.clone(), users: HashMap::new(),});
/// let instances = fakehub.githubs();
/// let created = instances.get(1).expect("Failed to get GitHub");
/// assert_that!(created.id, is(equal_to(expected)));
/// assert_that!(created.clone().url, is(equal_to(custom)));
/// assert_that!(instances.len(), is(equal_to(2)));
/// ```
pub struct Fakehub {
    /// GitHubs.
    pub hubs: HashMap<Uuid, GitHub>,
    /// Urled GitHubs.
    pub urled: HashMap<String, GitHub>,
}

impl Default for Fakehub {
    fn default() -> Fakehub {
        let mut hubs: HashMap<Uuid, GitHub> = HashMap::new();
        let id = Uuid::new_v4();
        let mut users: HashMap<String, User> = HashMap::new();
        let jeff = String::from("jeff");
        users.insert(jeff.clone(), User::new(jeff));
        hubs.insert(
            id,
            GitHub {
                id,
                url: String::from("https://github.com"),
                users,
            },
        );
        Fakehub {
            hubs: hubs.clone(),
            urled: hubs.into_values().map(|v| (v.clone().url, v)).collect(),
        }
    }
}

impl Fakehub {
    /// Add new GitHub
    pub fn add(&mut self, github: GitHub) {
        self.hubs.insert(github.id, github.clone());
        self.urled.insert(github.clone().url, github);
    }

    /// All GitHub instances.
    pub fn githubs(self) -> Vec<GitHub> {
        self.hubs
            .iter()
            .map(|(k, v)| GitHub {
                id: *k,
                url: v.url.clone(),
                users: v.users.clone(),
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
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn creates_default_fakehub() -> Result<()> {
        let fakehub = Fakehub::default();
        let default = fakehub
            .urled
            .get("https://github.com")
            .expect("Failed to get GitHub");
        assert_that!(default.id.is_nil(), is(equal_to(false)));
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
            users: HashMap::new(),
        });
        let github = fakehub.urled.get(&url).expect("Failed to get GitHub");
        assert_that!(github.id, is(equal_to(expected)));
        Ok(())
    }

    #[test]
    fn returns_all_github_instances_after_add() -> Result<()> {
        let mut fakehub = Fakehub::default();
        let custom = String::from("https://test.github.com");
        fakehub.add(GitHub {
            id: Uuid::new_v4(),
            url: custom.clone(),
            users: HashMap::new(),
        });
        let instances = fakehub.githubs();
        assert_that!(instances.len(), is(equal_to(2)));
        Ok(())
    }

    #[test]
    fn returns_default_instance() -> Result<()> {
        let fakehub = Fakehub::default();
        let instances = fakehub.githubs();
        let github = instances.first().expect("Failed to get GitHub");
        let users = github.clone().users();
        let user = users.first().expect("Failed to get user");
        assert_that!(&github.url, is(equal_to("https://github.com")));
        assert_that!(&user.username, is(equal_to("jeff")));
        Ok(())
    }

    #[test]
    fn returns_github_with_users() -> Result<()> {
        let fakehub = Fakehub::default();
        let github = fakehub
            .urled
            .get("https://github.com")
            .expect("Failed to get GitHub");
        let users = github.clone().users();
        let user = users.first().expect("Failed to get user");
        assert_that!(&user.username, is(equal_to("jeff")));
        Ok(())
    }
}

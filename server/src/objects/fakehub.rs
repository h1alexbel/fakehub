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

/// Fakehub. Fake GitHub platform.
///
/// To use `main` GitHub:
/// ```
/// use hamcrest::assert_that;
/// use hamcrest::{equal_to, is, HamcrestMatcher};
/// use server::objects::fakehub::Fakehub;
///
/// let fakehub = Fakehub::default();
/// let github = fakehub.main();
/// let jeff = github.user("jeff").expect("Failed to get user");
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
/// let mut fakehub = Fakehub::default();
/// let custom = String::from("google");
/// let expected = Uuid::new_v4();
/// fakehub.add(GitHub {id: expected, name: custom.clone(), users: HashMap::new(),});
/// let created = fakehub.browser.get(&custom).expect("Failed to get GitHub");
/// assert_that!(created.id, is(equal_to(expected)));
/// assert_that!(created.clone().name, is(equal_to(custom)));
/// ```
#[derive(Clone)]
pub struct Fakehub {
    /// GitHubs.
    pub hubs: HashMap<Uuid, GitHub>,
    /// Prefixed GitHubs.
    pub browser: HashMap<String, GitHub>,
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
                name: String::from("main"),
                users,
            },
        );
        Fakehub {
            hubs: hubs.clone(),
            browser: hubs.into_values().map(|v| (v.clone().name, v)).collect(),
        }
    }
}

impl Fakehub {
    /// Add new GitHub
    pub fn add(&mut self, github: GitHub) {
        self.hubs.insert(github.id, github.clone());
        self.browser.insert(github.clone().name, github);
    }

    /// All GitHub instances.
    pub fn githubs(self) -> Vec<GitHub> {
        self.hubs
            .iter()
            .map(|(k, v)| GitHub {
                id: *k,
                name: v.name.clone(),
                users: v.users.clone(),
            })
            .collect()
    }

    /// Main GitHub.
    pub fn main(&self) -> &GitHub {
        self.browser.get("main").expect("Failed to get main GitHub")
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
        let default = fakehub.browser.get("main").expect("Failed to get GitHub");
        assert_that!(default.id.is_nil(), is(equal_to(false)));
        Ok(())
    }

    #[test]
    fn adds_github() -> Result<()> {
        let mut fakehub = Fakehub::default();
        let url = String::from("google");
        let expected = Uuid::new_v4();
        fakehub.add(GitHub {
            id: expected,
            name: url.clone(),
            users: HashMap::new(),
        });
        let github = fakehub.browser.get(&url).expect("Failed to get GitHub");
        assert_that!(github.id, is(equal_to(expected)));
        Ok(())
    }

    #[test]
    fn returns_all_github_instances_after_add() -> Result<()> {
        let mut fakehub = Fakehub::default();
        let custom = String::from("myStartup");
        fakehub.add(GitHub {
            id: Uuid::new_v4(),
            name: custom.clone(),
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
        assert_that!(&github.name, is(equal_to("main")));
        assert_that!(&user.username, is(equal_to("jeff")));
        Ok(())
    }

    #[test]
    fn returns_github_with_users() -> Result<()> {
        let fakehub = Fakehub::default();
        let github = fakehub.browser.get("main").expect("Failed to get GitHub");
        let users = github.clone().users();
        let user = users.first().expect("Failed to get user");
        assert_that!(&user.username, is(equal_to("jeff")));
        Ok(())
    }

    #[test]
    fn returns_main_github() -> Result<()> {
        let fakehub = Fakehub::default();
        let github = fakehub.main();
        let users = github.clone().users();
        assert_that!(github.clone().id.is_nil(), is(equal_to(false)));
        assert_that!(&github.name, is(equal_to("main")));
        assert_that!(users.len(), is(equal_to(1)));
        Ok(())
    }
}

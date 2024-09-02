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
#[derive(Clone)]
pub struct Fakehub {
    /// GitHub.
    pub github: GitHub,
}

impl Default for Fakehub {
    fn default() -> Fakehub {
        let mut users: HashMap<String, User> = HashMap::new();
        let jeff = String::from("jeff");
        users.insert(jeff.clone(), User::new(jeff));
        Fakehub {
            github: GitHub {
                id: Uuid::new_v4(),
                name: String::from("main"),
                users,
            },
        }
    }
}

// @todo #124:90min Think about GitHub Enterprise GitHubs feature.
//  Let's think about how we can simulate multiple GitHub instances like GitHub
//  platform does with GitHub Enterprise feature. Fakehub should have multiple
//  GitHubs inside, and user can pick to which GitHub he wants store the
//  testing data.
impl Fakehub {
    /// Main GitHub.
    pub fn main(self) -> GitHub {
        self.github
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::fakehub::Fakehub;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[test]
    fn returns_default_fakehub_instance() -> Result<()> {
        let fakehub = Fakehub::default();
        let default = fakehub.main();
        assert_that!(default.id.is_nil(), is(equal_to(false)));
        Ok(())
    }

    #[test]
    fn returns_default_github() -> Result<()> {
        let fakehub = Fakehub::default();
        let github = fakehub.main();
        let users = github.clone().users();
        let user = users.first().expect("Failed to get user");
        assert_that!(&github.name, is(equal_to("main")));
        assert_that!(&user.username, is(equal_to("jeff")));
        Ok(())
    }
}

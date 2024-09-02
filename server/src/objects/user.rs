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
use anyhow::Result;
use log::info;
use serde::{Deserialize, Serialize};

/// GitHub user.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// Username.
    pub username: String,
}

impl User {
    /// New user.
    ///
    /// # Fields
    ///
    /// * `username`: Username
    ///
    /// # Examples
    ///
    /// ```
    /// use server::objects::user::User;
    /// let jeff = User::new(String::from("jeff123"));
    /// ```
    pub fn new(username: String) -> User {
        User { username }
    }
}

impl User {
    /// Register user in GitHub.
    /// `github` GitHub
    /// /// Register user in GitHub.
    ///```
    /// use server::objects::fakehub::Fakehub;
    /// use server::objects::user::User;
    /// let fakehub = Fakehub::default();
    /// let mut github = fakehub.main().clone();
    /// User::new(String::from("foo")).register_in(&mut github).expect("Failed to register user");
    ///```
    pub fn register_in(&self, github: &mut GitHub) -> Result<(), String> {
        match github.user(&self.username) {
            Some(u) => Err(format!("User with login @{} already exists!", u.username)),
            None => {
                github.add_user(self.clone());
                info!("New user is here. Hello @{}", self.username);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::fakehub::Fakehub;
    use crate::objects::user::User;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[test]
    fn returns_username() -> Result<()> {
        let expected = "jeff";
        let jeff = User::new(String::from(expected));
        assert_that!(jeff.username, is(equal_to(String::from(expected))));
        Ok(())
    }

    #[test]
    fn registers_in_github() -> Result<()> {
        let fakehub = Fakehub::default();
        let mut github = fakehub
            .browser
            .get("main")
            .expect("Failed to get GitHub")
            .clone();
        let foo = String::from("foo");
        User::new(foo.clone())
            .register_in(&mut github)
            .expect("Failed to register user");
        let pulled = github.users.get(&foo).expect("Failed to get user");
        assert_that!(pulled.clone().username, is(equal_to(foo)));
        Ok(())
    }

    #[should_panic(expected = "Failed to register user")]
    #[test]
    fn panics_when_already_registered() {
        let fakehub = Fakehub::default();
        let mut github = fakehub
            .browser
            .get("main")
            .expect("Failed to get GitHub")
            .clone();
        User::new(String::from("jeff"))
            .register_in(&mut github)
            .expect("Failed to register user");
    }
}

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
use crate::objects::user::User;
use std::collections::HashMap;
use uuid::Uuid;

/// GitHub.
#[derive(Clone)]
pub struct GitHub {
    /// GitHub ID.
    pub id: Uuid,
    /// GitHub URL.
    pub name: String,
    /// Users inside.
    pub users: HashMap<String, User>,
}

impl GitHub {
    /// Add user to GitHub.
    /// `user` User
    pub fn add_user(&mut self, user: User) {
        self.users.insert(user.clone().login, user);
    }

    /// User.
    pub fn user(&self, login: &str) -> Option<&User> {
        self.users.get(login)
    }

    /// Users in GitHub.
    pub fn users(self) -> Vec<User> {
        self.users.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::github::GitHub;
    use crate::objects::user::User;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn adds_user() -> Result<()> {
        let mut github = GitHub {
            id: Uuid::new_v4(),
            name: String::from("bar"),
            users: HashMap::new(),
        };
        let expected = String::from("jeff");
        github.add_user(User::new(expected.clone()));
        let users = github.users();
        let user = users.first().expect("Failed to get user");
        assert_that!(&user.login, is(equal_to(&expected)));
        Ok(())
    }

    #[test]
    fn returns_user_by_login() -> Result<()> {
        let mut github = GitHub {
            id: Uuid::new_v4(),
            name: String::from("bar"),
            users: HashMap::new(),
        };
        let expected = "foo";
        github.add_user(User::new(String::from(expected)));
        let foo = github.user(expected).expect("Failed to get user");
        assert_that!(&foo.login, is(equal_to(expected)));
        Ok(())
    }
}

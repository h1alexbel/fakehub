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
use serde::{Deserialize, Serialize};

/// Repo.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Repo {
    /// Repo name.
    pub name: String,
    /// Private or not?
    pub private: bool,
}

impl Repo {
    /// New repo.
    /// `name` Repo name
    /// `private` Private repo or not
    pub fn new(name: String, private: bool) -> Repo {
        Repo { name, private }
    }

    /// New public repo.
    /// `name` Repo name
    pub fn public(name: String) -> Repo {
        Repo {
            name,
            private: false,
        }
    }

    /// Create for.
    pub fn create_for(self, owner: &mut User) {
        owner.repos.push(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::fakehub::FakeHub;
    use crate::objects::repo::Repo;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[test]
    fn creates_repo() -> Result<()> {
        let fakehub = FakeHub::default();
        let github = fakehub.main();
        let mut jeff = github.user("jeff").expect("Failed to get user").clone();
        Repo::new(String::from("foo"), false).create_for(&mut jeff);
        assert_that!(jeff.repos.len(), is(equal_to(1)));
        Ok(())
    }
}

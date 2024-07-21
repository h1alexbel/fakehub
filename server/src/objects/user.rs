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
use anyhow::Result;
use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_xml_rs::to_string;

/// GitHub user.
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Username.
    pub(crate) username: String,
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

// @todo #17:40min Apply XMLed user to the <users/> node in storage.
//  We should apply XMLed user to the <users> XML node in storage. First we
//  need to check that user with provided name does not exist, and only then
//  apply it to the storage. Keep in mind that application function in the
//  storage should be thread-safe (as well as #xml function). Don't forget to
//  create unit tests that prove that.
impl User {
    /// Save user.
    pub async fn save(self) -> Result<()> {
        info!("saving user @{}", self.username);
        let xml = to_string(&self)?;
        debug!("XMLed user: {}", xml);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
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
}

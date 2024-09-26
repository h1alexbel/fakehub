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
use crate::handlers::node_id::NodeId;
use crate::objects::github::GitHub;
use crate::objects::user::User;
use chrono::{DateTime, Utc};
use serde_json::{Number, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Fakehub. Fake GitHub platform.
///
/// To use `main` GitHub:
/// ```
/// use hamcrest::assert_that;
/// use hamcrest::{equal_to, is, HamcrestMatcher};
/// use fakehub_server::objects::fakehub::FakeHub;
///
/// let mut fakehub = FakeHub::default();
/// let github = fakehub.main();
/// let locked = github.lock().expect("Failed to lock GitHub");
/// let jeff = locked.user("jeff").expect("Failed to get user");
/// assert_that!(&jeff.login, is(equal_to("jeff")));
/// ```
#[derive(Clone)]
pub struct FakeHub {
    /// GitHub.
    pub github: Arc<Mutex<GitHub>>,
    /// When it started.
    pub started: DateTime<Utc>,
    /// The address.
    pub address: String,
}

impl Default for FakeHub {
    fn default() -> FakeHub {
        FakeHub {
            github: Arc::new(Mutex::new(create_github())),
            started: Utc::now(),
            address: String::from("localhost"),
        }
    }
}

/// Create GitHub.
fn create_github() -> GitHub {
    let mut users: HashMap<String, User> = HashMap::new();
    let name = String::from("jeff");
    let mut jeff = User::new(name.clone());
    jeff.extra
        .insert(String::from("id"), Value::Number(Number::from(1)));
    let mut second = User::new(String::from("h1alexbel"));
    second
        .extra
        .insert(String::from("id"), Value::Number(Number::from(2)));
    users.insert(String::from("h1alexbel"), second);
    users.insert(name, jeff);
    GitHub {
        id: Uuid::new_v4(),
        name: String::from("main"),
        users,
    }
}

// @todo #124:90min Think about GitHub Enterprise GitHubs feature.
//  Let's think about how we can simulate multiple GitHub instances like GitHub
//  platform does with GitHub Enterprise feature. Fakehub should have multiple
//  GitHubs inside, and user can pick to which GitHub he wants store the
//  testing data.
impl FakeHub {
    /// New.
    pub fn new(started: DateTime<Utc>) -> FakeHub {
        FakeHub {
            github: Arc::new(Mutex::new(create_github())),
            started,
            address: String::from("localhost"),
        }
    }

    /// Create with address.
    pub fn with_addr(address: String) -> FakeHub {
        FakeHub {
            github: Arc::new(Mutex::new(create_github())),
            started: Utc::now(),
            address,
        }
    }

    /// Main GitHub.
    pub fn main(&self) -> Arc<Mutex<GitHub>> {
        Arc::clone(&self.github)
    }

    /// Coordinates.
    pub fn coords(&self) -> String {
        format!(
            "{};node:{}",
            self.address,
            NodeId { from: self.started }.as_string()
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::fakehub::FakeHub;
    use anyhow::Result;
    use chrono::{TimeZone, Utc};
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[test]
    fn returns_default_fakehub_instance() -> Result<()> {
        let fakehub = FakeHub::default();
        let github = fakehub.main();
        let locked = github.lock().expect("Failed to lock");
        assert_that!(locked.id.is_nil(), is(equal_to(false)));
        Ok(())
    }

    #[test]
    fn returns_default_github() -> Result<()> {
        let fakehub = FakeHub::default();
        let github = fakehub.main();
        let locked = github.lock().expect("Failed to lock");
        let users = locked.clone().users();
        assert_that!(&locked.name, is(equal_to("main")));
        assert_that!(users.len(), is(equal_to(2)));
        Ok(())
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn returns_coords() -> Result<()> {
        let fakehub = FakeHub::new(Utc.with_ymd_and_hms(2024, 9, 1, 9, 10, 11).unwrap());
        assert_that!(
            fakehub.coords(),
            is(equal_to(String::from(
                "localhost;node:305be946d516494d20c7c10f6d0020f9"
            )))
        );
        Ok(())
    }
}

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
use crate::handlers::cursor::Cursor;
use crate::handlers::sh_cursor::ShCursor;
use crate::objects::github::GitHub;
use crate::objects::repo::Repo;
use anyhow::Result;
use log::info;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};

/// GitHub user.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    /// Login, a.k.a. username.
    pub login: String,
    /// Repos.
    pub repos: Vec<Repo>,
    /// Extra information.
    pub extra: Map<String, Value>,
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
        User {
            login: username,
            repos: vec![],
            extra: Map::new(),
        }
    }

    /// Register user in GitHub.
    /// `github` GitHub
    /// /// Register user in GitHub.
    ///```
    /// use server::objects::fakehub::FakeHub;
    /// use server::objects::user::User;
    ///
    /// let fakehub = FakeHub::default();
    /// let mut github = fakehub.main().clone();
    /// User::new(String::from("foo")).register_in(&mut github).expect("Failed to register user");
    ///```
    // @todo #137:30min Create NodeId object that will give MD5 hash.
    //  Almost in any response, including user, we should display Node ID as
    //  GitHub API does. Looks like they use MD5 for it. Let's do the same, but
    //  we need only one that hash, because only one instance can be run. Let's
    //  hash UTC datetime when server was bootstrapped.
    // @todo #137:35min Pass port to the base cursor. Now we hardcode the
    //  address localhost:3000, however we need to handle generic ports that
    //  users can set.
    pub fn register_in(&mut self, github: &mut GitHub) -> Result<(), String> {
        match github.user(&self.login) {
            Some(u) => Err(format!("User with login @{} already exists!", u.login)),
            None => {
                let cursor = Cursor {
                    base: String::from("localhost:3000"),
                };
                let id = rand::thread_rng().gen_range(0..100_000_000);
                self.extra
                    .insert(String::from("node_id"), Value::String(String::from("")));
                self.extra
                    .insert(String::from("id"), Value::Number(Number::from(id)));
                self.extra.insert(
                    String::from("avatar_url"),
                    Value::String(format!("u/{}?v=4", id)),
                );
                self.extra
                    .insert(String::from("gravatar_id"), Value::String(String::from("")));
                self.extra.insert(
                    String::from("url"),
                    Value::String(
                        ShCursor::new(cursor.clone(), format!("users/{}", self.login))
                            .as_string(),
                    ),
                );
                self.extra.insert(
                    String::from("html_url"),
                    Value::String(
                        ShCursor::new(cursor.clone(), self.login.to_string()).as_string(),
                    ),
                );
                self.extra.insert(
                    String::from("followers_url"),
                    Value::String(
                        ShCursor::new(
                            cursor.clone(),
                            format!("users/{}/followers", self.login),
                        )
                        .as_string(),
                    ),
                );
                let following = ShCursor::new(
                    cursor.clone(),
                    format!("users/{}/following", self.login),
                )
                .as_string();
                self.extra.insert(
                    String::from("following_url"),
                    Value::String(format!(
                        "[{}{{/other_user}}]({}%7B/other_user%7D)",
                        following, following
                    )),
                );
                let gists =
                    ShCursor::new(cursor.clone(), format!("users/{}/gists", self.login))
                        .as_string();
                self.extra.insert(
                    String::from("gists_url"),
                    Value::String(format!(
                        "[{}{{/gist_id}}]({}%7B/gist_id%7D)",
                        gists, gists
                    )),
                );
                let starred = ShCursor::new(
                    cursor.clone(),
                    format!("users/{}/starred", self.login),
                )
                .as_string();
                self.extra.insert(
                    String::from("starred_url"),
                    Value::String(format!(
                        "[{}{{/owner}}{{/repo}}]({}%7B/owner%7D%7B/repo%7D)",
                        starred, starred
                    )),
                );
                self.extra.insert(
                    String::from("subscriptions_url"),
                    Value::String(
                        ShCursor::new(
                            cursor.clone(),
                            format!("users/{}/subscriptions", self.login),
                        )
                        .as_string(),
                    ),
                );
                self.extra.insert(
                    String::from("organizations_url"),
                    Value::String(
                        ShCursor::new(
                            cursor.clone(),
                            format!("users/{}/orgs", self.login),
                        )
                        .as_string(),
                    ),
                );
                self.extra.insert(
                    String::from("repos_url"),
                    Value::String(
                        ShCursor::new(
                            cursor.clone(),
                            format!("users/{}/repos", self.login),
                        )
                        .as_string(),
                    ),
                );
                let events =
                    ShCursor::new(cursor.clone(), format!("users/{}/events", self.login))
                        .as_string();
                self.extra.insert(
                    String::from("events_url"),
                    Value::String(format!(
                        "[{}{{/privacy}}]({}%7B/privacy%7D)",
                        events, events
                    )),
                );
                self.extra.insert(
                    String::from("received_events_url"),
                    Value::String(
                        ShCursor::new(
                            cursor,
                            format!("users/{}/received_events", self.login),
                        )
                        .as_string(),
                    ),
                );
                self.extra
                    .insert(String::from("type"), Value::String(String::from("User")));
                self.extra
                    .insert(String::from("site_admin"), Value::Bool(false));
                self.extra.insert(
                    String::from("name"),
                    Value::String(String::from("FakeHub user")),
                );
                self.extra.insert(String::from("company"), Value::Null);
                self.extra.insert(String::from("blog"), Value::Null);
                self.extra.insert(String::from("location"), Value::Null);
                self.extra.insert(String::from("email"), Value::Null);
                self.extra.insert(String::from("hireable"), Value::Null);
                self.extra.insert(String::from("bio"), Value::Null);
                self.extra
                    .insert(String::from("twitter_username"), Value::Null);
                self.extra
                    .insert(String::from("public_repos"), Value::Number(Number::from(0)));
                self.extra
                    .insert(String::from("public_gists"), Value::Number(Number::from(0)));
                self.extra
                    .insert(String::from("followers"), Value::Number(Number::from(0)));
                self.extra
                    .insert(String::from("following"), Value::Number(Number::from(0)));
                let now = chrono::offset::Utc::now();
                self.extra
                    .insert(String::from("created_at"), Value::String(now.to_string()));
                self.extra
                    .insert(String::from("updated_at"), Value::String(now.to_string()));
                github.add_user(self.clone());
                info!("Registered @{}", self.login);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::fakehub::FakeHub;
    use crate::objects::user::User;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[test]
    fn returns_username() -> Result<()> {
        let expected = "jeff";
        let jeff = User::new(String::from(expected));
        assert_that!(jeff.login, is(equal_to(String::from(expected))));
        Ok(())
    }

    #[test]
    fn registers_in_github() -> Result<()> {
        let fakehub = FakeHub::default();
        let mut github = fakehub.main();
        let foo = String::from("foo");
        User::new(foo.clone())
            .register_in(&mut github)
            .expect("Failed to register user");
        let pulled = github.users.get(&foo).expect("Failed to get user");
        assert_that!(pulled.clone().login, is(equal_to(foo)));
        Ok(())
    }

    #[should_panic(expected = "Failed to register user")]
    #[test]
    fn panics_when_already_registered() {
        let fakehub = FakeHub::default();
        let mut github = fakehub.main();
        User::new(String::from("jeff"))
            .register_in(&mut github)
            .expect("Failed to register user");
    }

    #[test]
    fn registers_with_extra() -> Result<()> {
        let fakehub = FakeHub::default();
        let mut github = fakehub.main();
        User::new(String::from("foo"))
            .register_in(&mut github)
            .expect("Failed to register user");
        let user = github.users.get("foo").expect("Failed to get user");
        let url = user.extra.get("url").expect("Failed to read property");
        assert_that!(url.as_str(), is(equal_to(Some("localhost:3000/users/foo"))));
        assert_that!(user.extra.len(), is(equal_to(31)));
        Ok(())
    }
}

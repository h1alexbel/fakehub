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
use serde_json::{Map, Value};

/// JSON User.
#[derive(Serialize, Deserialize)]
pub struct JsonUser {
    origin: User,
}

impl JsonUser {
    /// New.
    pub fn new(origin: User) -> JsonUser {
        JsonUser { origin }
    }

    /// Print it as JSON document.
    pub fn as_json(self) -> Value {
        let mut map = Map::new();
        map.insert(String::from("login"), Value::from(self.origin.login));
        for (k, v) in self.origin.extra.into_iter() {
            map.insert(k, v);
        }
        Value::Object(map)
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::json::json_user::JsonUser;
    use crate::objects::user::User;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};
    use serde_json::Value;

    #[test]
    fn prints_json() -> Result<()> {
        let jeff = "jeff";
        let mut base = User::new(String::from(jeff));
        let url = "testing";
        base.extra
            .insert(String::from("url"), Value::String(String::from(url)));
        let user = JsonUser::new(base);
        let value = user.as_json();
        assert_that!(value["login"].as_str(), is(equal_to(Some(jeff))));
        assert_that!(value["url"].as_str(), is(equal_to(Some(url))));
        Ok(())
    }
}

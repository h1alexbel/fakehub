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
use chrono::{DateTime, Utc};

/// Node ID.
#[derive(Clone)]
pub struct NodeId {
    /// From.
    pub from: DateTime<Utc>,
}

impl NodeId {
    /// Print it as string.
    pub fn as_string(self) -> String {
        format!("{:x}", md5::compute(self.from.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::objects::node_id::NodeId;
    use anyhow::Result;
    use chrono::{TimeZone, Utc};
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[test]
    #[allow(clippy::unwrap_used)]
    fn creates_node_id() -> Result<()> {
        let hash = NodeId {
            from: Utc.with_ymd_and_hms(2024, 9, 1, 9, 10, 11).unwrap(),
        }
            .as_string();
        assert_that!(hash, is(equal_to(String::from("305be946d516494d20c7c10f6d0020f9"))));
        Ok(())
    }
}

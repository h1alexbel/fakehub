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

/// Slashed cursor with complete value.
/// ```
/// use hamcrest::assert_that;
/// use hamcrest::{equal_to, is, HamcrestMatcher};
/// use server::handlers::cursor::Cursor;
/// use server::handlers::sh_cursor::ShCursor;
///
/// let url = ShCursor::new(Cursor::new(String::from("http://localhost:3000")), String::from("repos")).as_string();
/// assert_that!(url, is(equal_to(String::from("http://localhost:3000/repos"))));
/// ```
pub struct ShCursor {
    origin: Cursor,
    complete: String,
}

impl ShCursor {
    /// New.
    pub fn new(origin: Cursor, complete: String) -> ShCursor {
        ShCursor { origin, complete }
    }

    /// Format as string.
    pub fn as_string(self) -> String {
        format!("{}/{}", self.origin.as_string(), self.complete)
    }
}

#[cfg(test)]
mod tests {

    use crate::handlers::cursor::Cursor;
    use crate::handlers::sh_cursor::ShCursor;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[test]
    fn prints_formatted() -> Result<()> {
        let url = ShCursor::new(
            Cursor {
                base: String::from("http://localhost:3000"),
            },
            String::from("repos"),
        )
        .as_string();
        assert_that!(
            url,
            is(equal_to(String::from("http://localhost:3000/repos")))
        );
        Ok(())
    }
}

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
use std::fs;
use std::path::Path;

/// Read LaTeX template.
/// @todo #41:60min Add function for appending new content into the template.
///  We need to create new function that will append input into the template,
///  thus it will build a detailed report.
/// # Arguments
///
/// * `path`: Template path
///
/// returns: String
///
/// # Examples
///
/// ```
/// use crate::fakehub_server::report::latex::template;
/// let content = template("resources/report.tex");
/// print!("{content}")
/// ```
pub fn template(path: &str) -> String {
    return fs::read_to_string(Path::new(path)).expect("template should be read from");
}

#[cfg(test)]
mod tests {
    use crate::report::latex::template;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[test]
    // @todo #41:60min Add support of @ExtendsWith from JUnit in order to pass expected as test parameter.
    //  We should use extensions in order to pass expected as parameters into
    //  test. If there is no crate with such functionality - let's develop and
    //  release one.
    fn returns_template_content() -> Result<()> {
        let content = template("resources/report.tex");
        let expected = r"\usepackage{to-be-determined}
\documentclass[12pt]{article}
\begin{document}

\section*{Fakehub report}
\tbd{History: TBD}
\end{document}
";
        assert_that!(content, is(equal_to(String::from(expected))));
        Ok(())
    }
}

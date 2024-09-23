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
#[allow(clippy::question_mark_used)]
#[cfg(test)]
mod routes_its {
    use anyhow::Result;
    #[allow(deprecated)]
    // @todo #79:25min Import assert_that!() macro like in lib.rs instead of assert_that.
    //  For some reason we can't use '#[macro_use] extern crate hamcrest;'
    //  inside this mod. We should figure out why and replace deprecated import
    //  with required macro.
    use hamcrest::{assert_that, equal_to, is, HamcrestMatcher};
    use fakehub_server::Server;

    #[tokio::test]
    #[ignore]
    // @todo #79:35min Enable integration test case for home endpoint.
    //  For now this test does not work, because we can't abort the server.
    //  Let's implement it first inside lib.rs::Server struct and then enable this case.
    // @todo #79:25min Check home response from body.
    //  We should check that returned home body contains exactly the same response that
    //  in resources/home.json (expected). Don't forget this puzzle.
    async fn returns_home() -> Result<()> {
        let port = 1234;
        Server::new(port)
            .start()
            .await
            .expect("Server failed to start");
        let response = reqwest::Client::new()
            .get(format!("http://localhost:{}/", port))
            .send()
            .await?;
        assert_that!(response.status().as_u16(), is(equal_to(200)));
        Ok(())
    }
}

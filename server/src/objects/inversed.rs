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
use std::collections::HashMap;
use std::hash::Hash;

/// Inverse HashMap.
/// `map` Input HashMap
pub fn inversed<X, Y>(map: HashMap<X, Y>) -> HashMap<Y, X>
where
    X: Clone + Eq + Hash,
    Y: Clone + Eq + Hash,
{
    map.into_iter().map(|(k, v)| (v, k)).collect()
}

#[cfg(test)]
mod tests {
    use crate::objects::inversed::inversed;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};
    use std::collections::HashMap;
    use uuid::Uuid;

    #[test]
    fn inverses_map() -> Result<()> {
        let mut map: HashMap<Uuid, String> = HashMap::new();
        let value = String::from("test");
        let key = Uuid::new_v4();
        map.insert(key, value.clone());
        let inversed = inversed(map);
        let reverse = inversed.get(&value).expect("Failed to get value");
        assert_that!(*reverse, is(equal_to(key)));
        Ok(())
    }
}

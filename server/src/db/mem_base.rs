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
use rusqlite::{Connection, Params, Statement};

/// In memory database.
pub struct MemBase {
    connection: Connection,
}

impl MemBase {
    /// New base.
    pub fn new() -> MemBase {
        MemBase {
            connection: Connection::open_in_memory().expect("Can not open a connection"),
        }
    }

    /// Base from connection.
    pub fn conn(connection: Connection) -> MemBase {
        MemBase { connection }
    }

    /// Execute SQL.
    /// `sql`: SQL statement
    /// `params`: Parameters
    pub fn exec<P: Params>(&self, sql: &str, params: P) -> usize {
        self.connection
            .execute(sql, params)
            .expect(&format!("Failed to to execute {}", sql))
    }

    /// Prepare statement.
    /// `sql`: SQL statement
    pub fn prep(&self, sql: String) -> Statement {
        self.connection
            .prepare(&sql)
            .expect(&format!("Failed to prepare {}", sql))
    }
}

#[cfg(test)]
mod tests {

    use crate::db::mem_base::MemBase;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};
    use rusqlite::Connection;

    #[test]
    fn connects_and_prepares() -> Result<()> {
        let base = MemBase::new();
        let mut statement = base.prep(String::from("SELECT 1 + 1"));
        let mut rows = statement.query([]).expect("Failed to obtain rows");
        let mut out: Vec<usize> = Vec::new();
        while let Some(row) = rows.next()? {
            out.push(row.get(0).expect("Failed to read row"));
        }
        let result = out.get(0).expect("Failed to obtain result");
        assert_that!(*result, is(equal_to(2)));
        Ok(())
    }

    #[test]
    fn executes_sql() -> Result<()> {
        let base = MemBase::conn(
            Connection::open_in_memory().expect("Can not open a connection"),
        );
        base.exec("CREATE TABLE test (id INTEGER INTEGER PRIMARY KEY);", ());
        let rows = base.exec("INSERT INTO test (id) VALUES (?1)", [&1]);
        assert_that!(rows, is(equal_to(1)));
        Ok(())
    }
}

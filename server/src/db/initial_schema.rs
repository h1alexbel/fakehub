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
use crate::db::mem_base::MemBase;

// @todo #121:60min Use migration and versioning tool for initializing database
//  schema. With manual schema initializing, we don't track the changes and can't
//  easily rollback by removing some files. Would be much more convenient to
//  operate with .xml/.yml/.sql files that will be read and applied by some
//  versioning tool. However, keep in mind that tool should be integrated on
//  the source code level, and don't depend on previously installed software
//  on user's system. I suggest to try the following setup: Docker client with
//  <a href="https://github.com/liquibase/liquibase/blob/master/liquibase-maven-plugin/pom.xml">liquibase-maven-plugin</a>.
//  With docker we should be able to install Liquibase, and with plugin we
//  should be able to parse and update changelog of migrations. 
pub fn initialize_schema(base: &MemBase) {
    base.exec(
        "
        CREATE TABLE github
         (id INTEGER PRIMARY KEY AUTOINCREMENT, url VARCHAR (256) UNIQUE NOT NULL)",
        (),
    );
    base.exec(
        "INSERT INTO github (url) VALUES (:url)",
        ["https://github.com"],
    );
    base.exec(
        "
    CREATE TABLE user (
     id INTEGER PRIMARY KEY AUTOINCREMENT,
     login CHARACTER VARYING (128) UNIQUE NOT NULL,
     github REFERENCES github (id) NOT NULL
    )",
        (),
    );
    base.exec(
        "INSERT INTO user (login, github) VALUES (:login, (SELECT id FROM github WHERE url = 'https://github.com'));",
        ["jeff"]
    );
}

#[cfg(test)]
mod tests {

    use crate::db::initial_schema::initialize_schema;
    use crate::db::mem_base::MemBase;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};

    #[test]
    // @todo #121:35min Remove code duplication in the logic of reading data
    //  from MemBase. Now we duplicating the same logic for reading data from
    //  MemBase. Let's encapsulate this logic into query() so it can wrap
    //  MemBase#prep function. This puzzle should affect all tests that use
    //  MemBase to read data. 
    fn initializes_tables() -> Result<()> {
        let base = MemBase::new();
        initialize_schema(&base);
        let mut statement = base.prep(String::from(
            "SELECT name FROM sqlite_master WHERE type = 'table' AND name = 'github';",
        ));
        let mut rows = statement.query([]).expect("Failed to obtain rows");
        let mut out: Vec<String> = Vec::new();
        while let Some(row) = rows.next()? {
            out.push(row.get(0).expect("Failed to read row"));
        }
        let result = out.get(0).expect("Failed to obtain result");
        assert_that!(result, is(equal_to("github")));
        Ok(())
    }

    #[test]
    fn reads_default_github_url() -> Result<()> {
        let base = MemBase::new();
        initialize_schema(&base);
        let mut statement =
            base.prep(String::from("SELECT url FROM github WHERE url = :public"));
        let public = "https://github.com";
        let mut rows = statement.query([public]).expect("Failed to obtain rows");
        let mut out: Vec<String> = Vec::new();
        while let Some(row) = rows.next()? {
            out.push(row.get(0).expect("Failed to read row"));
        }
        let result = out.get(0).expect("Failed to obtain result");
        assert_that!(result, is(equal_to(public)));
        Ok(())
    }

    #[test]
    fn reads_jeff() -> Result<()> {
        let base = MemBase::new();
        initialize_schema(&base);
        let mut statement =
            base.prep(String::from("SELECT login FROM user WHERE login = :login"));
        let login = "jeff";
        let mut rows = statement.query([login]).expect("Failed to obtain rows");
        let mut out: Vec<String> = Vec::new();
        while let Some(row) = rows.next()? {
            out.push(row.get(0).expect("Failed to read row"));
        }
        let result = out.get(0).expect("Failed to obtain result");
        assert_that!(result, is(equal_to(login)));
        Ok(())
    }

    #[test]
    fn checks_jeff_github() -> Result<()> {
        let base = MemBase::new();
        initialize_schema(&base);
        let mut statement =
            base.prep(String::from("SELECT github FROM user WHERE login = :login"));
        let mut rows = statement.query(["jeff"]).expect("Failed to obtain rows");
        let mut out: Vec<usize> = Vec::new();
        while let Some(row) = rows.next()? {
            out.push(row.get(0).expect("Failed to read row"));
        }
        let result = *out.get(0).expect("Failed to obtain result");
        assert_that!(result, is(equal_to(1)));
        Ok(())
    }
}

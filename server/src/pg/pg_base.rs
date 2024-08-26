use postgres::{Client, NoTls, Row};

pub struct PgBase {
    client: Client,
}

impl PgBase {
    pub fn new(host: String, port: usize) -> PgBase {
        let client = Client::connect(
            &format!("postgresql://postgres:postgres@{}:{}", host, port),
            NoTls,
        )
        .expect(&format!("Can not connect to {}:{}", host, port));
        PgBase { client }
    }

    pub fn query(mut self, sql: &str) -> Vec<Row> {
        self.client
            .query(sql, &[])
            .expect(&format!("Can not query with: {}", sql))
    }

    pub fn exec(mut self, sql: &str) {
        self.client
            .batch_execute(sql)
            .expect(&format!("Can not execute sql: {}", sql));
    }
}

#[cfg(test)]
mod tests {
    use crate::pg::pg_base::PgBase;
    use anyhow::Result;
    use hamcrest::{equal_to, is, HamcrestMatcher};
    use testcontainers_modules::postgres;
    use testcontainers_modules::testcontainers::runners::AsyncRunner;

    #[tokio::test]
    async fn connects() -> Result<()> {
        let container = postgres::Postgres::default().start().await?;
        let port = container.get_host_port_ipv4(5432).await?;
        let rows = tokio::task::spawn_blocking(move || {
            PgBase::new(String::from("localhost"), port as usize).query("SELECT 1 + 1")
        }).await?;
        let first = &rows[0].get(0);
        assert_that!(*first, is(equal_to(2)));
        Ok(())
    }
}

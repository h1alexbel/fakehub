use crate::objects::json::json_user::JsonUser;
use crate::ServerConfig;
use axum::extract::State;
use axum::Json;
use serde_json::Value;

/// All GitHub users, ordered by their id.
/// See: <a href="https://api.github.com/users">GitHub implementation</a>.
pub async fn users(State(config): State<ServerConfig>) -> Json<Vec<Value>> {
    let fakehub = config.fakehub;
    let github = fakehub.main();
    Json(
        github
            .users()
            .iter()
            .map(|u| JsonUser::new(u.clone()).as_json())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use crate::handlers::users::users;
    use crate::objects::fakehub::FakeHub;
    use crate::ServerConfig;
    use anyhow::Result;
    use axum::extract::State;

    #[tokio::test]
    #[allow(clippy::question_mark_used)]
    async fn returns_users() -> Result<()> {
        let users = users(State(ServerConfig {
            fakehub: FakeHub::default(),
        }))
        .await;
        let print = users.0;
        println!("{}", serde_json::to_string_pretty(&print)?);
        Ok(())
    }
}

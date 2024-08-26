use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::helpers::get_env;

pub async fn initialize_pgpool() -> PgPool {
    let pg_host = get_env("POSTGRES_HOST");
    let pg_user = get_env("POSTGRES_USER");
    let pg_password =
        get_env("POSTGRES_PASSWORD");
    let pg_db = get_env("POSTGRES_DB");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(
            format!(
                "postgres://{}:{}@{}/{}",
                pg_user, pg_password, pg_host, pg_db
            )
            .as_str(),
        )
        .await
        .expect("Could not connect to Postgres DB")
}

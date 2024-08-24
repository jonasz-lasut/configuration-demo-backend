use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn initialize_pgpool() -> PgPool {
  let pg_host = std::env::var("POSTGRES_HOST").expect("No POSTGRES_HOST variable provided");
  let pg_user = std::env::var("POSTGRES_USER").expect("No POSTGRES_USER variable provided");
  let pg_password =
      std::env::var("POSTGRES_PASSWORD").expect("No POSTGRES_PASSWORD variable provided");
  let pg_db = std::env::var("POSTGRES_DB").expect("No POSTGRES_DB variable provided");

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

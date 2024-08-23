use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use sqlx::{FromRow, PgPool};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pg_host = std::env::var("POSTGRES_HOST").expect("No POSTGRES_HOST variable provided");
    let pg_user = std::env::var("POSTGRES_USER").expect("No POSTGRES_USER variable provided");
    let pg_password = std::env::var("POSTGRES_PASSWORD").expect("No POSTGRES_PASSWORD variable provided");
    let pg_db = std::env::var("POSTGRES_PASSWORD").expect("No POSTGRES_PASSWORD variable provided");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(format!("postgres://{}:{}@{}/{}", pg_user, pg_password, pg_host, pg_db).as_str())
        .await?;

    let state = Repository { pool: pool };

    let app = Router::new()
        .route("/todo/:id", get(get_todo))
        .route("/todo", get(get_all_todo))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn get_todo(
    Path(id): Path<i32>,
    State(state): State<Repository>
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string()))
    }
}

async fn get_all_todo(
    State(state): State<Repository>
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(&state.pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string()))
    }
}

#[derive(Clone)]
struct Repository {
    pool: PgPool,
}

#[derive(Deserialize)]
struct TodoNew {
    pub note: String,
}

#[derive(Serialize, FromRow)]
struct Todo {
    pub id: i32,
    pub note: String,
}

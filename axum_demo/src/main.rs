use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use core::result::Result::Ok;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, PgPool};
use tokio::net::TcpListener;
use tracing::{info, instrument};

#[tokio::main]
#[instrument]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("simple_api_rust_axum=trace")
        .init();

    let pg_host = std::env::var("POSTGRES_HOST").expect("No POSTGRES_HOST variable provided");
    let pg_user = std::env::var("POSTGRES_USER").expect("No POSTGRES_USER variable provided");
    let pg_password =
        std::env::var("POSTGRES_PASSWORD").expect("No POSTGRES_PASSWORD variable provided");
    let pg_db = std::env::var("POSTGRES_DB").expect("No POSTGRES_DB variable provided");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            format!(
                "postgres://{}:{}@{}/{}",
                pg_user, pg_password, pg_host, pg_db
            )
            .as_str(),
        )
        .await?;

    let state = Repository { pool: pool };

    let app = Router::new()
        .route("/", get(get_all_todo).post(create_todo))
        .route("/:id", get(get_todo).delete(delete_todo).post(update_todo))
        .with_state(state);

    let bind_addr = "0.0.0.0:8080";
    let listener = TcpListener::bind(bind_addr).await.unwrap();
    info!("🚀 Starting server at {}", bind_addr);
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn get_todo(
    Path(id): Path<i32>,
    State(state): State<Repository>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn get_all_todo(
    State(state): State<Repository>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(&state.pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn create_todo(
    State(state): State<Repository>,
    Json(data): Json<TodoNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (note) VALUES ($1, $2) RETURNING id, note, done",
    )
    .bind(&data.note)
    .bind(&data.done)
    .fetch_one(&state.pool)
    .await
    {
        Ok(todo) => Ok((StatusCode::CREATED, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn delete_todo(
    Path(id): Path<i32>,
    State(state): State<Repository>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Todo>("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

async fn update_todo(
    Path(id): Path<i32>,
    State(state): State<Repository>,
    Json(data): Json<TodoNew>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as::<_, Todo>("UPDATE todos SET note=$1 done=$2 WHERE id = $3")
        .bind(&data.note)
        .bind(&data.done)
        .bind(id)
        .fetch_one(&state.pool)
        .await
    {
        Ok(todo) => Ok((StatusCode::OK, Json(todo))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[derive(Clone)]
struct Repository {
    pool: PgPool,
}

#[derive(Deserialize)]
struct TodoNew {
    pub note: String,
    pub done: Option<bool>,
}

#[derive(Serialize, FromRow)]
struct Todo {
    pub id: i32,
    pub note: String,
    pub done: Option<bool>,
}

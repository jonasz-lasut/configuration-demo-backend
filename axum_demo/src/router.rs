use axum::{
  routing::{get, delete},
  Router,
};
use tower_http::services::ServeDir;
use sqlx::PgPool;

use crate::handlers;

#[derive(Clone)]
pub struct Repository {
    pub pool: PgPool,
}

pub fn init_router(db: PgPool) -> Router {
  let state = Repository { pool: db };

  Router::new()
      .route("/", get(handlers::home))
      .route("/healthz", get(handlers::healthz))
      .route("/styles.css", get(handlers::styles))
      .route("/todos", get(handlers::get_all_todo))
      .route("/todos/:id", delete(handlers::delete_todo))//.post(handlers::update_todo)
      .route("/todos/create", get(handlers::todo_create_handler).post(handlers::create_todo))
      .route("/todos/edit/:id", get(handlers::todo_update_handler).post(handlers::update_todo))
      .nest_service(
        "/assets",
        ServeDir::new(format!("{}/assets",  std::env::current_dir().unwrap().to_str().unwrap())), // Serve static assets
      )
      .with_state(state)
      .fallback(handlers::handler_404)
}

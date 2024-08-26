use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Form, Json,
};
use core::result::Result::Ok;

use crate::errors::ApiError;
use crate::models::{GenericResponse, Todo, TodoNew, TodoUpdate};
use crate::router::Repository;
use crate::templates;

pub async fn home() -> impl IntoResponse {
    templates::HelloTemplate
}

pub async fn healthz() -> Result<impl IntoResponse, ()> {
    Ok((
        StatusCode::OK,
        Json(GenericResponse {
            message: String::from("healthy"),
        }),
    ))
}

pub async fn styles() -> Result<impl IntoResponse, ApiError> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../templates/styles.css").to_owned())?;

    Ok(response)
}

pub async fn get_all_todo(State(state): State<Repository>) -> Result<impl IntoResponse, ApiError> {
    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos")
        .fetch_all(&state.pool)
        .await?;

    Ok(templates::Records { todos })
}

pub async fn todo_create_handler() -> impl IntoResponse {
    templates::TodoCreationModalTemplate
}

pub async fn create_todo(
    State(state): State<Repository>,
    Form(data): Form<TodoNew>,
) -> Result<impl IntoResponse, ApiError> {
    sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (note, status) VALUES ($1, $2) RETURNING id, note, status",
    )
    .bind(&data.note)
    .bind(false)
    .fetch_one(&state.pool)
    .await?;

    Ok(templates::HelloTemplate)
}

pub async fn delete_todo(
    Path(id): Path<i32>,
    State(state): State<Repository>,
) -> Result<impl IntoResponse, ApiError> {
    sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    Ok(templates::HelloTemplate)
}

pub async fn handler_404() -> impl IntoResponse {
    templates::Error404Template {
        reason: "Nothing to see here".to_string(),
    }
}

pub async fn todo_update_handler(
    Path(id): Path<i32>,
    State(state): State<Repository>,
) -> Result<impl IntoResponse, ApiError> {
    let todo = sqlx::query_as::<_, Todo>("SELECT * FROM todos WHERE id = $1")
        .bind(id)
        .fetch_one(&state.pool)
        .await?;

    Ok(templates::TodoUpdateModalTemplate { todo: todo })
}

pub async fn update_todo(
    Path(id): Path<i32>,
    State(state): State<Repository>,
    Form(data): Form<TodoUpdate>,
) -> Result<impl IntoResponse, ApiError> {
    sqlx::query("UPDATE todos SET note = $1, status = $2 WHERE id = $3")
        .bind(&data.note)
        .bind(&data.status)
        .bind(id)
        .execute(&state.pool)
        .await?;

    Ok(templates::HelloTemplate)
}

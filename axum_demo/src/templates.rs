use askama::Template;

use crate::models;

#[derive(Template)]
#[template(path = "layout/index.html")]
pub struct HelloTemplate {
    pub title: String
}

#[derive(Template)]
#[template(path = "todos/todos.html")]
pub struct Records {
    pub todos: Vec<models::Todo>,
}

#[derive(Template)]
#[template(path = "partial/todo.html")]
pub struct TodoNewTemplate {
    pub todo: models::Todo,
}

#[derive(Template)]
#[template(path = "partial/todo_create.html")]
pub struct TodoCreationModalTemplate;

#[derive(Template)]
#[template(path = "partial/todo_update.html")]
pub struct TodoUpdateModalTemplate {
    pub todo: models::Todo,
}

#[derive(Template)]
#[template(path = "error/404.html")]
pub struct Error404Template {
    pub reason: String,
}

use crate::handlers::{root::root, todo_hander::{create_todo, get_todos, get_todos_by_id, change_status, delete_todo}};
use crate::schema::todo::TodoStorage;

use std::fs::File;

use axum::{
    Router,
    routing::{delete, get, post, put},
};

use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct AppState {
    pub container: Arc<Mutex<TodoStorage>>
}

impl AppState {
    pub fn new() -> Self{
        Self { container: Arc::new(Mutex::new(TodoStorage::new())) }
    }
}

pub async fn axum_router() -> Router {
    let state = AppState::new();

    let app = Router::new()
        // `GET /` goes to `root`
        .route("/", get(root))
        .route("/add-todo", post(create_todo))
        .route("/all-todos", get(get_todos))
        .route("/get-todo", get(get_todos_by_id))
        .route("/change-status", put(change_status))
        .route("/delete-todo", delete(delete_todo))
        .with_state(state);
        // `POST /users` goes to `create_user`
        // .route("/users", post(create_user));
    app
}
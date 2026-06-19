use crate::schema::todo::{Todo, Status, CreateTodoPayload, GetTodoPayload, UpdateTodoPayload, DeletTodoPayload};
use crate::routes::AppState;
use uuid::Uuid;

use axum::extract::State;
use axum::{Json, http::StatusCode};

// #[axum::debug_handler]
pub async fn create_todo(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUserPayload` type
    State(state): State<AppState>,
    Json(payload): Json<CreateTodoPayload>
) -> Result<(StatusCode, Json<Todo>), String> {
    // insert your application logic here
    let todo = Todo {
        id: Uuid::new_v4(),
        name: payload.name,
        status: Status::Started,
    };

    let mut store = state.container.lock().unwrap();
    match store.add(todo.clone()) {
        Ok(()) => (),
        Err(_) => return Err("Failed to add file".to_string())
    };


    // this will be converted into a JSON response
    // with a status code of `201 Created`
    Ok((StatusCode::CREATED, Json(todo)))
}



pub async fn get_todos(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUserPayload` type
    State(state): State<AppState>,
) -> (StatusCode, Json<Vec<Todo>>) {
    // insert your application logic here
    
    let todos = {
        let store = state.container.lock().unwrap();
        store.get_all().to_vec()
    };
        
    // this will be converted into a JSON response
    // with a status code of `201 Created`
    (StatusCode::FOUND, Json(todos))
}

pub async fn get_todos_by_id(    
    State(state): State<AppState>,
    Json(payload): Json<GetTodoPayload>
) -> (StatusCode, Json<Todo>) {
    
    let todo = {
        let store = state.container.lock().unwrap();
        store.get_todo_by_id(payload.id).unwrap().clone()
    };

    
    (StatusCode::FOUND, Json(todo))
}

pub async fn change_status(
    State(state): State<AppState>,
    Json(payload): Json<UpdateTodoPayload>
) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    
    let todo = {
        let mut store = state.container.lock().unwrap();
        match store.change_status(payload.id, payload.status) {
            Ok(todo) => todo,
            Err(_) => return Err(StatusCode::NOT_FOUND)
        }
    };
    
    Ok((StatusCode::FOUND, Json(todo)))
}


pub async fn delete_todo(
    State(state): State<AppState>,
    Json(payload): Json<DeletTodoPayload>
) -> Result<(StatusCode, Json<Todo>), StatusCode> {

    let todo = {
        let mut store = state.container.lock().unwrap();

        match store.delete_todo(payload.id) {
            Ok(t) => t,
            Err(_) => return Err(StatusCode::NOT_FOUND)
        }
    };

    Ok((StatusCode::FOUND, Json(todo)))
}
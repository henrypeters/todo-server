// use serde::{Deserialize, Serialize};
// // use uuid::Uuid;

// #[derive(Serialize, Deserialize)]
// pub struct CreateTodoPayload {
//     pub name: String,
//     pub status: u8
// }

// #[derive(Serialize)]
// pub struct TodoStorage<'a> {
//     todos: &'a Vec<Todo>
// }

// #[derive(Clone, Serialize)]
// pub struct Todo {
//     // pub id: Uuid,
//     pub name: String,
//     pub status: Status,
// }

// #[derive(Clone, Deserialize, Serialize)]
// pub enum Status {
//     Started,
//     Completed,
//     InvalidEntry

// }

// impl Status {
//     pub fn map_id_to_status(x: u8) -> Self {
//         match x {
//             1 => Status::Started,
//             2 => Status::Completed,
//             _ => Status::InvalidEntry
//         }
//     }
// }


// impl<'a> TodoStorage<'a> {
//     pub fn new(todos: &'a Vec<Todo>) -> Self{
//         Self { 
//             todos
//         }
//     }

//     pub fn add(&mut self, todo: Todo) {
//         self.todos.push(todo);
//     }

//     pub fn get_all(&self) -> Vec<&'a Todo> {
//         for todo in self.todos {
//             todo
//         }
//     }
// }
































use std::io::ErrorKind::Other;

use axum::http::status;
use serde::{Deserialize, Serialize};
use tokio::sync::futures::OwnedNotified;
use uuid::Uuid;

use crate::utils::util::{load_todos, save_todos};

#[derive(Deserialize)]
pub struct CreateTodoPayload {
    pub name: String
}

#[derive(Deserialize)]
pub struct GetTodoPayload {
    pub id: Uuid
}

#[derive(Deserialize)]
pub struct UpdateTodoPayload {
    pub id: Uuid,
    pub status: u8
}

#[derive(Deserialize)]
pub struct DeletTodoPayload {
    pub id: Uuid
}

#[derive(Serialize)]
pub struct TodoStorage {
    pub todos: Vec<Todo>
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub name: String,
    pub status: Status,
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub enum Status {
    Started,
    Completed,
    InvalidEntry

}

impl Status {
    pub fn map_id_to_status(x: u8) -> Self {
        match x {
            1 => Status::Started,
            2 => Status::Completed,
            _ => Status::InvalidEntry
        }
    }
}


impl TodoStorage {
    pub fn new() -> Self{
        let todos = load_todos();

        Self { 
            todos: todos
        }
    }

    pub fn add(&mut self, todo: Todo) -> std::io::Result<()> {
        self.todos.push(todo);
        let new_collection = &self.todos;

        println!("Todos in memory {}", self.todos.len());

        save_todos(new_collection)
    }

    pub fn get_all(&self) -> &[Todo] {
        &self.todos
    }

    pub fn save_to_file(&self, todos: &[Todo]) -> std::io::Result<()> {
        save_todos(todos)
    }

    pub fn get_todo_by_id(&self, id: Uuid) -> Option<&Todo> {
        let index = self.todos.iter().position(|todo| todo.id == id);
         
        match index {
            Some(t) => {
                let todo = &self.todos[t];
                Some(todo)
            },
            None => None
        }
    }

    pub fn change_status(&mut self, id: Uuid, status: u8) -> std::io::Result<Todo> {
        let new_status = Status::map_id_to_status(status);

        let updated_todo = {
            let todo = self.todos.iter_mut().find(|todo| todo.id == id).ok_or(std::io::Error::other("Cannot find todo"))?;

            if todo.status == new_status {
                return Err(std::io::Error::other("Status already set"))
            }

            todo.status = new_status;
            todo.clone()
        };

        self.save_to_file(&self.todos)?;    // match self.save_to_file() {
                                            // Ok(()) => (),
                                            // Err(e) => return Err(std::io::Error())
                                            // },

        Ok(updated_todo)


    }

    pub fn delete_todo(&mut self, id: Uuid) -> std::io::Result<Todo> {

        let index = self.todos.iter().position(|todo| todo.id == id).ok_or(std::io::Error::other("todo index not found"))?;
        let todo = {
            let todo = &self.todos[index];

            todo.clone()
        };
        
        self.todos.remove(index);

        self.save_to_file(&self.todos)?;

        Ok(todo)
    }

    // pub fn change_status()
    
}
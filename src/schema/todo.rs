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
































use axum::http::status;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

#[derive(Clone, Serialize)]
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
        Self { 
            todos: vec![]
        }
    }

    pub fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    pub fn get_all(&self) -> &[Todo] {
        &self.todos
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

    pub fn change_status(&mut self, id: Uuid, status: u8) -> Option<Todo> {
        let index = self.todos.iter().position(|todo| todo.id == id);
         
        match index {
            Some(t) => {
                let todo = &mut self.todos[t];

                if todo.status != Status::map_id_to_status(status) {
                    todo.status = Status::map_id_to_status(status);
                    Some(todo.clone())
                }else {
                    None
                }
            },
            None => None
        }
    }

    pub fn delete_todo(&mut self, id: Uuid) -> Option<Todo> {
        let index = self.todos.iter().position(|todo| todo.id == id);

        match index {
            Some(t) => {
                let todo = &mut self.todos.remove(t);

                Some(todo.clone())
            },
            None => None
        }
    }

    // pub fn change_status()
    
}
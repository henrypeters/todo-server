use std::fs;
use std::path::Path;
use serde_json;

use crate::schema::todo::{Todo};

const FILE_PATH: &str = "/home/henry-peters/Desktop/drips_project/yg/todo_server/data/todo.json";

pub fn load_todos() -> Vec<Todo> {
    if !Path::new(FILE_PATH).exists() {
        return vec![]
    }   

    // let content = fs::read_to_string(FILE_PATH).unwrap_or_else(|_| "[]".to_string());
    let content = fs::read_to_string(FILE_PATH).expect("Failed to read file");

    // serde_json::from_str(&content).unwrap_or_else(|_| vec![])
    serde_json::from_str(&content).expect("Failed to deserialize")
}

pub fn save_todos(todos: &[Todo]) -> std::io::Result<()> {
    println!("Saving {} todos", todos.len());

    let json = serde_json::to_string_pretty(todos).expect("Failed to serialize todos");
    println!("Json: \n{}", json);

    let result = fs::write(FILE_PATH, json);

    println!("Write result: {:?}", result);

    result
}
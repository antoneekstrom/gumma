use std::{collections::HashMap, sync::Mutex};

use actix_web::{self, web, Result};
use serde::{Deserialize, Serialize};

type Id = u32;

pub fn todo_config(cfg: &mut web::ServiceConfig) {
    let state = web::Data::new(State::new());
    cfg.app_data(state.clone())
        .service(todos_get)
        .service((todo_add, todo_delete, todo_toggle));
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum TodoState {
    Checked,
    Unchecked,
}

impl TodoState {
    pub fn toggle(&self) -> Self {
        match self {
            TodoState::Checked => TodoState::Unchecked,
            TodoState::Unchecked => TodoState::Checked,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct Todo {
    title: String,
    state: TodoState,
}

#[derive(Debug)]
struct State {
    todos: Mutex<HashMap<Id, Todo>>,
}

impl State {
    pub fn new() -> Self {
        State {
            todos: Mutex::new(HashMap::new()),
        }
    }
}

#[actix_web::get("")]
async fn todos_get(data: web::Data<State>) -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body(format!("{:?}", data.todos))
}

#[derive(Deserialize)]
struct TodoToggleQuery {
    id: Id,
}

#[actix_web::post("/toggle/{id}")]
async fn todo_toggle(path: web::Path<TodoToggleQuery>, data: web::Data<State>) -> Result<String> {
    if let Ok(mut todos) = data.todos.lock() {
        let todo = todos.get_mut(&path.id).unwrap();
        todo.state = todo.state.toggle();
    }

    Ok(format!("Toggled todo with id {}", &path.id))
}

#[actix_web::post("/delete/{id}")]
async fn todo_delete(path: web::Path<TodoToggleQuery>, data: web::Data<State>) -> Result<String> {
    if let Ok(mut todos) = data.todos.lock() {
        todos.remove(&path.id);
    }

    Ok(format!("Toggled todo with id {}", &path.id))
}

#[actix_web::post("/add")]
async fn todo_add(todo_json: web::Json<Todo>, data: web::Data<State>) -> Result<String> {
    let mut todos = data.todos.lock().unwrap();

    let id = todos.len() as Id;
    let todo = todo_json.clone();

    todos.insert(id, todo);

    Ok(serde_json::to_string(&todo_json).unwrap())
}

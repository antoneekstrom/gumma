use std::{collections::HashMap, sync::Mutex};

use actix_web::{
    self,
    middleware::{NormalizePath, TrailingSlash},
    web, Result,
};
use serde::Deserialize;

type Id = u32;

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

async fn todos(data: web::Data<State>) -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body(format!("{:?}", data.todos))
}

#[derive(Deserialize)]
struct TodoToggleQuery {
    id: Id,
}

#[actix_web::get("/toggle/{id}")]
async fn todo_toggle(path: web::Path<TodoToggleQuery>, data: web::Data<State>) -> Result<String> {
    if let Ok(mut todos) = data.todos.lock() {
        let todo = todos.get_mut(&path.id).unwrap();
        todo.state = todo.state.toggle();
    }

    Ok(format!("Toggled todo with id {}", &path.id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Define one state that is shared between each process
    let state = web::Data::new(State::new());

    actix_web::HttpServer::new(move || {
        let todos_service = web::scope("/todos")
            .app_data(state.clone())
            .service(web::resource("").to(todos))
            .service(
                web::scope("")
                    .service(todo_toggle)
                    .wrap(NormalizePath::new(TrailingSlash::Trim)),
            );

        actix_web::App::new().service(todos_service)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

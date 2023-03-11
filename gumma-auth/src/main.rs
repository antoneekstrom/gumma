use actix_web::{self, web};

#[derive(Clone, Debug)]
enum Todo {
    Checked(String),
    Unchecked(String),
}

#[derive(Clone, Debug)]
struct State {
    todos: Vec<Todo>,
}

impl State {
    pub fn new() -> Self {
        State { todos: Vec::new() }
    }
}

async fn todos(data: web::Data<State>) -> impl actix_web::Responder {
    actix_web::HttpResponse::Ok().body(format!("{:?}", data.todos))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Define one state that is shared between each process
    let state = web::Data::new(State::new());

    actix_web::HttpServer::new(move || {
        let todos_service = web::scope("/todos")
            .app_data(state.clone())
            .route("/", web::get().to(todos));
        actix_web::App::new().service(todos_service)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

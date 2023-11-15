use actix_web::{get, web, App, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod utils;
use utils::response::{create_response_message, HttpStatus};

mod api;
use api::todolist::service::services;

struct AppState {
    todolist_entries: Mutex<Vec<TodoListEntry>>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct TodoListEntry {
    id: i32,
    date: i64,
    title: String,
}

#[get("/")]
async fn index() -> impl Responder {
    create_response_message(
        "My first api with Rust + Actix".to_string(),
        HttpStatus::Ok as u16,
    )
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        todolist_entries: Mutex::new(vec![]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind(("127.0.0.1", 3000))?
    .run()
    .await
}

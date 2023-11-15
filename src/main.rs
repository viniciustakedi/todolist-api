use actix_web::{ get, web, App, HttpServer };
use serde::{ Deserialize, Serialize };
use std::sync::Mutex;

mod api;
use api::todolist::service::services;

struct AppState {
    todolist_entries: Mutex<Vec<TodoListEntry>>
}

#[derive(Serialize, Deserialize, Clone)]
struct TodoListEntry {
    id: i32,
    date: i64,
    title: String,
}

#[get("/")]
async fn index() -> String {
    "My first api with Rust".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState{
        todolist_entries: Mutex::new(vec![])
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
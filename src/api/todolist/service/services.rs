use crate::api::todolist::model::models::{CreateEntryData, UpdateEntryData};
use crate::utils::response::{create_response_data, create_response_message, HttpStatus};
use crate::{AppState, TodoListEntry};
use actix_web::{delete, get, post, put, web, Responder};

#[get("/todolist/entries")]
async fn get_entries(data: web::Data<AppState>) -> impl Responder {
    let data: Vec<TodoListEntry> = data.todolist_entries.lock().unwrap().to_vec();
    let total: u32 = data.len() as u32;

    create_response_data::<Vec<_>>(
        data,
        total,
        "Sucesso ao obter os items".to_string(),
        HttpStatus::Ok as u16,
    )
    .await
}

#[post("/todolist/entries")]
async fn create_entry(
    data: web::Data<AppState>,
    param_obj: web::Json<CreateEntryData>,
) -> impl Responder {
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    let mut max_id: i32 = 0;

    for i in 0..todolist_entries.len() {
        if todolist_entries[i].id > max_id {
            max_id = todolist_entries[i].id;
        }
    }

    todolist_entries.push(TodoListEntry {
        id: max_id + 1,
        title: param_obj.title.clone(),
        date: param_obj.date,
    });

    create_response_message(
        "Tarefa criada com sucesso!".to_string(),
        HttpStatus::Created as u16,
    )
    .await
}

#[put("/todolist/entries/{id}")]
async fn update_entry(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    param_obj: web::Json<UpdateEntryData>,
) -> impl Responder {
    let id: i32 = path.into_inner();
    let mut todolist_entries = data.todolist_entries.lock().unwrap();

    for i in 0..todolist_entries.len() {
        if todolist_entries[i].id == id {
            todolist_entries[i].title = param_obj.title.clone();
            break;
        }
    }

    create_response_message(
        format!("Tarefa {} atualizada com sucesso!", id).to_string(),
        HttpStatus::Ok as u16,
    )
    .await
}

#[delete("/todolist/entries/{id}")]
async fn delete_entry(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let mut todolist_entries = data.todolist_entries.lock().unwrap();
    let id = path.into_inner();

    *todolist_entries = todolist_entries
        .to_vec()
        .into_iter()
        .filter(|x| x.id != id)
        .collect();

    create_response_message(
        format!("Tarefa {} deletada com sucesso!", id),
        HttpStatus::Ok as u16,
    )
    .await
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_entries)
        .service(create_entry)
        .service(update_entry)
        .service(delete_entry);
}

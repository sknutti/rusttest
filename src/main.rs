use actix_files::{self as fs, NamedFile};
use actix_web::*;
use serde_derive::*;
use std::sync::Mutex;

#[derive(Serialize, Debug)]
struct State {
    todo_items: Mutex<Vec<String>>,
}

#[post("/api/todo")]
async fn add_data(value: String, data: web::Data<State>) -> HttpResponse {
    let mut items = data.todo_items.lock().unwrap();
    items.push(value.replace("\"", ""));
    HttpResponse::Ok().json(items.clone())
}

#[patch("/api/todo")]
async fn clear_data(data: web::Data<State>) -> HttpResponse {
    let mut items = data.todo_items.lock().unwrap();
    *items = vec![];
    HttpResponse::Ok().json(items.clone())
}

#[delete("/api/todo")]
async fn remove_data(value: String, data: web::Data<State>) -> HttpResponse {
    let mut items = data.todo_items.lock().unwrap();
    let val = value.replace("\"", "");
    let index = items.iter().position(|x| *x == val);
    items.remove(index.unwrap());
    HttpResponse::Ok().json(items.clone())
}

#[get("/api/todo")]
async fn get_data(data: web::Data<State>) -> HttpResponse {
    let items = data.todo_items.lock().unwrap();
    HttpResponse::Ok().json(items.clone())
}

#[get("/")]
async fn page() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(State {
        todo_items: Mutex::new(vec![
            "This".to_string(),
            "Is".to_string(),
            "Working!".to_string(),
        ]),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(add_data)
            .service(remove_data)
            .service(clear_data)
            .service(get_data)
            .service(page)
            .service(fs::Files::new("/static", "./pkg").show_files_listing())
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

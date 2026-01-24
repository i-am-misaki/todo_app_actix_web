//! TODOアプリのルーティングとハンドラ定義
//!
//! - GET /todos
//! - POST /todos
//! - PUT /todo/edit
//! - POST /todo/delete
mod db;
mod handlers;
mod forms;
mod models;

use actix_web::{App, HttpServer, web, HttpResponse, Responder, get};
use actix_files::Files;
use dotenvy::dotenv;
use tera::Tera;

use db::init_db;
use handlers::todo::{get_todos, add_todo, delete_todo, edit_todo};
mod repository;



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


/// # Examples
///
/// ```
/// let x = 5;
/// ```
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let tera = Tera::new("templates/**/*").unwrap();
    let pool = init_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(pool.clone()))
            .service(hello)
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(add_todo))
            .route("/todo/delete", web::post().to(delete_todo))
            .route("/todo/edit", web::put().to(edit_todo))

            // ここに静的ファイルサービスを追加
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


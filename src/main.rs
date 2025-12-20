use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{Pool, Postgres};
use tera::{Tera, Context};
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;


mod db;
use db::init_db;


// 表示用
#[derive(Deserialize, Serialize)]
struct TodoForm {
    id: i32,
    task: String,
}

#[derive(Deserialize)]
struct TodoAddForm {
    task: String
}

// 削除用
#[derive(Deserialize)]
struct TodoDeleteForm {
    id: i32,
}



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}



async fn get_todos(tera: web::Data<Tera>, db: web::Data<Pool<Postgres>>) -> impl Responder {
    // id, task の取得
    let rows = sqlx::query!("SELECT id, task FROM todos")
        .fetch_all(db.get_ref())
        .await
        .unwrap();

    let tasks: Vec<TodoForm> = rows.into_iter()
                                .map(|r| TodoForm {
                                    id: r.id,
                                    task: r.task
                                })
                                .collect();

    let mut ctx = Context::new();
    ctx.insert("tasks", &tasks);
    let rendered = tera.render("index.html", &ctx).unwrap();

    HttpResponse::Ok().content_type("text/html").body(rendered)
}


async fn add_todo(form: web::Form<TodoAddForm>, db: web::Data<Pool<Postgres>>) -> impl Responder {
    sqlx::query!("INSERT INTO todos (task) VALUES ($1)", form.task)
        .execute(db.get_ref())
        .await
        .unwrap();

    HttpResponse::SeeOther()
        .append_header(("Location", "/todos"))
        .finish()
}

async fn delete_todo(form: web::Form<TodoDeleteForm>, db: web::Data<Pool<Postgres>>) -> impl Responder {
    sqlx::query!("DELETE FROM todos WHERE id = $1", form.id)
        .execute(db.get_ref())
        .await
        .unwrap();

    HttpResponse::SeeOther()
        .append_header(("Location", "/todos"))
        .finish()
}



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

            // ここに静的ファイルサービスを追加
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


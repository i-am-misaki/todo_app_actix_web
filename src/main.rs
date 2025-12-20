//! TODOアプリのルーティングとハンドラ定義
//!
//! - GET /todos
//! - POST /todos
//! - PUT /todo/edit
//! - POST /todo/delete

use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use sqlx::{Pool, Postgres};
use tera::{Tera, Context};
use serde::{Deserialize, Serialize};
use dotenvy::dotenv;


mod db;
use db::init_db;


/// TODO 作成・編集時に使用するフォームデータ
#[derive(Deserialize, Serialize)]
struct TodoForm {
    /// TODO ID
    id: i32,

    /// タスク内容
    task: String,
}

/// TODO 追加に使用するフォームデータ
#[derive(Deserialize)]
struct TodoAddForm {
    /// タスク内容
    task: String
}

// TODO 削除に使用するフォームデータ
#[derive(Deserialize)]
struct TodoDeleteForm {
    /// TODO ID
    id: i32,
}



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


/// TODO 一覧を取得して表示する。
///
/// # 処理内容
/// - todos テーブルから全件取得
/// - Tera テンプレートに渡して HTML を返却
///
/// # レスポンス
/// - 200 OK (text/html)
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


/// TODOを新規追加する。
///
/// フォームから受け取ったタスク内容を DB に保存し、
/// 一覧画面へリダイレクトする。
async fn add_todo(form: web::Form<TodoAddForm>, db: web::Data<Pool<Postgres>>) -> impl Responder {
    sqlx::query!("INSERT INTO todos (task) VALUES ($1)", form.task)
        .execute(db.get_ref())
        .await
        .unwrap();

    HttpResponse::SeeOther()
        .append_header(("Location", "/todos"))
        .finish()
}


/// TODOを削除する。
///
/// 指定された ID の TODO を削除する。
async fn delete_todo(form: web::Form<TodoDeleteForm>, db: web::Data<Pool<Postgres>>) -> impl Responder {
    sqlx::query!("DELETE FROM todos WHERE id = $1", form.id)
        .execute(db.get_ref())
        .await
        .unwrap();

    HttpResponse::SeeOther()
        .append_header(("Location", "/todos"))
        .finish()
}


/// TODOを編集する
///
/// # 引数
/// - id: 編集対象のTODO ID
/// - task: 更新後のタスク内容
async fn edit_todo(form: web::Form<TodoForm>, db: web::Data<Pool<Postgres>>) -> impl Responder {
    sqlx::query!("UPDATE todos SET task = $1 WHERE id = $2", form.task, form.id)
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
            .route("/todo/edit", web::put().to(edit_todo))

            // ここに静的ファイルサービスを追加
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}


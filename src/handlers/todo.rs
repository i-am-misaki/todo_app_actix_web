use actix_web::{web, HttpResponse, Responder};
use sqlx::{Pool, Postgres};
use tera::{Tera, Context};

use crate::forms::{TodoAddForm, TodoEditForm, TodoDeleteForm};
use crate::models::Todo;
use crate::repository::todo;



/// TODO 一覧を取得して表示する。
///
/// # 処理内容
/// - todos テーブルから全件取得
/// - Tera テンプレートに渡して HTML を返却
///
/// # レスポンス
/// - 200 OK (text/html)
pub async fn get_todos(tera: web::Data<Tera>, db: web::Data<Pool<Postgres>>) -> impl Responder {
    // id, task の取得
    let tasks: Vec<Todo> = todo::find_all(db.get_ref())
        .await
        .unwrap();

    let mut ctx = Context::new();
    ctx.insert("tasks", &tasks);
    let rendered = tera.render("index.html", &ctx).unwrap();

    HttpResponse::Ok().content_type("text/html").body(rendered)
}


/// TODOを新規追加する。
///
/// フォームから受け取ったタスク内容を DB に保存し、
/// 一覧画面へリダイレクトする。
pub async fn add_todo(form: web::Form<TodoAddForm>, db: web::Data<Pool<Postgres>>) -> impl Responder {
    todo::insert(db.get_ref(), &form.task)
        .await
        .unwrap();

    HttpResponse::SeeOther()
        .append_header(("Location", "/todos"))
        .finish()
}


/// TODOを削除する。
///
/// 指定された ID の TODO を削除する。
pub async fn delete_todo(form: web::Form<TodoDeleteForm>, db: web::Data<Pool<Postgres>>) -> impl Responder {
    todo::delete(db.get_ref(), form.id)
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
pub async fn edit_todo(form: web::Form<TodoEditForm>, db: web::Data<Pool<Postgres>>) -> impl Responder {
    todo::update(db.get_ref(), &form.id, &form.task)
        .await
        .unwrap();

    HttpResponse::SeeOther()
        .append_header(("Location", "/todos"))
        .finish()
}

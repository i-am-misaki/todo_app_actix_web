use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
// use std::sync::Mutex;
use std::io::{Write, BufRead, BufReader};
use std::fs::{OpenOptions, File};
use tera::{Tera, Context};
// use serde::Deserialize;


#[derive(serde::Deserialize)]
struct TodoForm {
    task: String,
}



#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}



async fn get_todos(tera: web::Data<Tera>) -> impl Responder {
    // tasks.txt を読み込む
    let file = File::open("tasks.txt").unwrap_or_else(|_| File::create("tasks.txt").unwrap());
    // |_| は「エラー内容は使わないよ」という意味の 無名関数（クロージャ）。
    // 「もし tasks.txt がなかったら、新しく作る（File::create）
    let reader = BufReader::new(file);
    let tasks: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    // Vec<String> は 「文字列（String）のリスト」 のこと。
    // Rust の Vec は 可変長配列（リスト） に近い概念
    // 「ファイルの各行を文字列に変換して、ベクタ（リスト）にまとめる」処理

    // Tera に tasks を渡す
    let mut ctx = Context::new();
    ctx.insert("tasks", &tasks);
    let rendered = tera.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}


async fn add_todos(form: web::Form<TodoForm>) -> impl Responder {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("tasks.txt")
        .unwrap();
    writeln!(file, "{}", form.task.trim()).unwrap();

    // POST後は /todos にリダイレクト
    HttpResponse::SeeOther()
    // HttpResponse::SeeOther()                          「HTTPステータス303 See Other」のレスポンスを作成
        .append_header(("Location", "/todos"))
        // .append_header(("Location", "/todos"))         Location ヘッダーとは「リダイレクト先のURL」を指定するもの
        .finish()
        // .finish()                                      レスポンスを完成させて返す
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(hello)
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(add_todos))

            // ここに静的ファイルサービスを追加
            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
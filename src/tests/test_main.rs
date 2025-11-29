
// use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
// // get, post     ルートを定義するための属性マクロ（#[get("/path")] のように使う）
// // web           ルーティングやリクエスト情報を扱うモジュール
// // App           サーバー内でルートやサービスを登録するためのコンテナ
// // HttpResponse  HTTPレスポンスを作る構造体
// // HttpServer    サーバーを立ち上げるための構造体
// // Responder     ハンドラ関数の戻り値に使える型（HTTPレスポンスに変換できるもの）


// // async fn      Actix Web は非同期実行（async/await）に対応しているので、全てのハンドラは async

// #[get("/healthcheck")]
// async fn hello() -> impl Responder {
//     HttpResponse::Ok().body("Hello world!")
// }

// #[post("/echo")]
// async fn echo(req_body: String) -> impl Responder {
//     HttpResponse::Ok().body(req_body)
// }

// async fn manual_hello() -> impl Responder {
//     HttpResponse::Ok().body("Hey there!")
// }


// #[actix_web::main]
// // async fn main() -> std::io::Result<()> {
// async fn test_main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//     // HttpServer::new(|| { ... })                           サーバーインスタンスを作成。
//     App::new()
//     // クロージャ内で App::new() を呼び、アプリケーション（ルーティングなど）を設定
//             .service(hello)
//             .service(echo)
//             // .service(hello) / .service(echo)               #[get] や #[post] で定義した関数をサービスとして登録
//             .route("/hey", web::get().to(manual_hello))
//             // .route("/hey", web::get().to(manual_hello))    手動でルートを追加。GET /hey にアクセスすると manual_hello() を呼ぶ
//     })
//     .bind(("127.0.0.1", 8080))?
//     // .bind(("127.0.0.1", 8080))                             サーバーのアドレスとポートを指定
//     .run()
//     .await
//     // .run().await                                           サーバーを起動してリクエスト待ち状態に入る。
// }

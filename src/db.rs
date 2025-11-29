use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub async fn init_db() -> Pool<Postgres> {

    // Postgres に接続して コネクションプールを作る
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();

    // マイグレーション実行
    sqlx::migrate!().run(&pool).await.unwrap();

    pool
}
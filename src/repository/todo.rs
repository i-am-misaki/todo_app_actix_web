use sqlx::{Pool, Postgres};

use crate::models::todo::Todo;


/// TODO 一覧を取得する。
///
/// todos テーブルから全件取得し、
/// ドメインモデル `Todo` の配列として返す。
pub async fn find_all(pool: &Pool<Postgres>) -> Result<Vec<Todo>, sqlx::Error>{
    let rows = sqlx::query!("SELECT id, task FROM todos")
        .fetch_all(pool)
        .await?;

    Ok(rows.into_iter()
        .map(|r| Todo {
            id: r.id,
            task: r.task,
        })
        .collect())
}



/// TODO を新規作成する。
///
/// 指定されたタスク内容を todos テーブルに登録する。
pub async fn insert(pool: &Pool<Postgres>, task: &str) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO todos (task) VALUES ($1)", task)
        .execute(pool)
        .await?;

    Ok(())
}


/// TODO を削除する。
///
/// 指定された ID の TODO を todos テーブルから削除する。
pub async fn delete(pool: &Pool<Postgres>, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .execute(pool)
        .await?;

    Ok(())
}



/// TODO を更新する。
///
/// 指定された ID の TODO のタスク内容を更新する。
pub async fn update(pool: &Pool<Postgres>, id: &i32, task: &str) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE todos SET task = $1 WHERE id = $2", task, id)
        .execute(pool)
        .await?;

    Ok(())
}

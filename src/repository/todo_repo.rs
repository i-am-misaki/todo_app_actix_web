use sqlx::{Pool, Postgres};

use crate::models::todo::Todo;


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


pub async fn insert(pool: &Pool<Postgres>, task: &str) -> Result<(), sqlx::Error> {
    sqlx::query!("INSERT INTO todos (task) VALUES ($1)", task)
        .fetch_all(pool)
        .await?;

    Ok(())
}


pub async fn delete(pool: &Pool<Postgres>, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .fetch_all(pool)
        .await?;

    Ok(())
}


pub async fn update(pool: &Pool<Postgres>, id: &i32, task: &str) -> Result<(), sqlx::Error> {
    sqlx::query!("UPDATE todos SET task = $1 WHERE id = $2", task, id)
        .fetch_all(pool)
        .await?;

    Ok(())
}

use serde::Serialize;

/// TODO 情報を表すモデル
#[derive(Serialize)]
pub struct Todo {
    pub id: i32,
    pub task: String,
}

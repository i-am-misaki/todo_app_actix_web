use serde::Deserialize;

/// TODO 追加フォーム
#[derive(Deserialize)]
pub struct TodoAddForm {
    pub task: String,
}

/// TODO 編集フォーム
#[derive(Deserialize)]
pub struct TodoEditForm {
    pub id: i32,
    pub task: String,
}

/// TODO 削除フォーム
#[derive(Deserialize)]
pub struct TodoDeleteForm {
    pub id: i32,
}


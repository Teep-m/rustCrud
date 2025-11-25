use serde::{Deserialize, Serialize};

/// Todo作成リクエストDTO
#[derive(Debug, Deserialize)]
pub struct CreateTodoDto {
    pub title: String,
    #[serde(default)]
    pub completed: bool,
}

/// Todo更新リクエストDTO
#[derive(Debug, Deserialize)]
pub struct UpdateTodoDto {
    pub title: Option<String>,
    pub completed: Option<bool>,
}

/// TodoレスポンスDTO
#[derive(Debug, Serialize)]
pub struct TodoResponseDto {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

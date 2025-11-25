use std::sync::Arc;
use crate::domain::todo::{Todo, TodoRepository};
use super::dto::{CreateTodoDto, UpdateTodoDto, TodoResponseDto};

/// Todoサービス
/// アプリケーションのユースケースを実装
pub struct TodoService {
    repository: Arc<dyn TodoRepository>,
}

impl TodoService {
    pub fn new(repository: Arc<dyn TodoRepository>) -> Self {
        Self { repository }
    }

    /// すべてのTodoを取得
    pub async fn get_all_todos(&self) -> Result<Vec<TodoResponseDto>, String> {
        let todos = self.repository.find_all().await?;
        Ok(todos.into_iter().map(Self::to_response_dto).collect())
    }

    /// IDでTodoを取得
    pub async fn get_todo_by_id(&self, id: i32) -> Result<TodoResponseDto, String> {
        let todo = self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| "Todoが見つかりません".to_string())?;
        
        Ok(Self::to_response_dto(todo))
    }

    /// Todoを作成
    pub async fn create_todo(&self, dto: CreateTodoDto) -> Result<TodoResponseDto, String> {
        let mut todo = Todo::new(dto.title)?;
        if dto.completed {
            todo.complete();
        }
        let saved_todo = self.repository.save(&todo).await?;
        Ok(Self::to_response_dto(saved_todo))
    }

    /// Todoを更新
    pub async fn update_todo(&self, id: i32, dto: UpdateTodoDto) -> Result<TodoResponseDto, String> {
        let mut todo = self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| "Todoが見つかりません".to_string())?;

        // タイトルの更新
        if let Some(title) = dto.title {
            todo.change_title(title)?;
        }

        // 完了状態の更新
        if let Some(completed) = dto.completed {
            if completed {
                todo.complete();
            } else {
                todo.uncomplete();
            }
        }

        let updated_todo = self.repository.update(&todo).await?;
        Ok(Self::to_response_dto(updated_todo))
    }

    /// Todoを削除
    pub async fn delete_todo(&self, id: i32) -> Result<(), String> {
        // 存在確認
        self.repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| "Todoが見つかりません".to_string())?;

        self.repository.delete(id).await
    }

    /// Todoエンティティをレスポンスdtoに変換
    fn to_response_dto(todo: Todo) -> TodoResponseDto {
        TodoResponseDto {
            id: todo.id().expect("保存されたTodoにはIDが必要です"),
            title: todo.title().to_string(),
            completed: todo.is_completed(),
        }
    }
}

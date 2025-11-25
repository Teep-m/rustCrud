use crate::domain::todo::{Todo, TodoRepository};
use crate::infrastructure::persistence::database::DbClient;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;

/// SurrealDB用のTodoレコード
#[derive(Debug, Serialize, Deserialize)]
struct TodoRecord {
    id: Option<Thing>,
    title: String,
    completed: bool,
}

impl From<TodoRecord> for Todo {
    fn from(record: TodoRecord) -> Self {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let id = record
            .id
            .map(|thing| {
                let id_str = thing.id.to_string();
                // まず整数としてパースを試みる
                id_str.parse::<i32>().unwrap_or_else(|_| {
                    // パースできない場合はハッシュ値を使用（SurrealDBのULID/UUID対応）
                    let mut hasher = DefaultHasher::new();
                    id_str.hash(&mut hasher);
                    let hash = hasher.finish();
                    // i32の正の範囲に収める（1から始まる）
                    ((hash % (i32::MAX as u64 - 1)) + 1) as i32
                })
            })
            .unwrap_or(1); // IDが無い場合のデフォルト値

        Todo::reconstruct(id, record.title, record.completed)
    }
}

/// SurrealDB実装のTodoリポジトリ
pub struct TodoRepositoryImpl {
    db: DbClient,
}

impl TodoRepositoryImpl {
    pub fn new(db: DbClient) -> Self {
        Self { db }
    }
}

#[async_trait]
impl TodoRepository for TodoRepositoryImpl {
    async fn find_all(&self) -> Result<Vec<Todo>, String> {
        let records: Vec<TodoRecord> = self
            .db
            .select("todos")
            .await
            .map_err(|e| format!("データベースエラー: {}", e))?;

        Ok(records.into_iter().map(Into::into).collect())
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Todo>, String> {
        let record: Option<TodoRecord> = self
            .db
            .select(("todos", id.to_string()))
            .await
            .map_err(|e| format!("データベースエラー: {}", e))?;

        Ok(record.map(Into::into))
    }

    async fn save(&self, todo: &Todo) -> Result<Todo, String> {
        #[derive(Serialize)]
        struct CreateTodo {
            title: String,
            completed: bool,
        }

        let new_todo = CreateTodo {
            title: todo.title().to_string(),
            completed: todo.is_completed(),
        };

        // createは単一レコードを返すので Option<TodoRecord> を取得し unwrap
        let created: Option<TodoRecord> = self
            .db
            .create("todos")
            .content(new_todo)
            .await
            .map_err(|e| format!("データベースエラー: {}", e))?;

        let created = created.ok_or_else(|| "作成に失敗しました".to_string())?;
        Ok(created.into())
    }

    async fn update(&self, todo: &Todo) -> Result<Todo, String> {
        let id = todo.id().ok_or("更新対象のTodoにIDが必要です")?;

        #[derive(Serialize)]
        struct UpdateTodo {
            title: String,
            completed: bool,
        }

        let update_data = UpdateTodo {
            title: todo.title().to_string(),
            completed: todo.is_completed(),
        };

        let updated: Option<TodoRecord> = self
            .db
            .update(("todos", id.to_string()))
            .content(update_data)
            .await
            .map_err(|e| format!("データベースエラー: {}", e))?;

        updated
            .map(Into::into)
            .ok_or_else(|| "更新に失敗しました".to_string())
    }

    async fn delete(&self, id: i32) -> Result<(), String> {
        let _: Option<TodoRecord> = self
            .db
            .delete(("todos", id.to_string()))
            .await
            .map_err(|e| format!("データベースエラー: {}", e))?;

        Ok(())
    }
}

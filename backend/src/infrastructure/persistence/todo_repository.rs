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
        use surrealdb::sql::Id;

        let id = record
            .id
            .map(|thing| {
                match &thing.id {
                    Id::String(s) => {
                        println!("🔧 ID変換: SurrealDB ID (String) = '{}'", s);
                        // 文字列を整数にパース
                        s.parse::<i32>().unwrap_or_else(|_| {
                            // パースできない場合はハッシュ値を使用（ULID/UUID対応）
                            let mut hasher = DefaultHasher::new();
                            s.hash(&mut hasher);
                            let hash = hasher.finish();
                            let final_id = ((hash % (i32::MAX as u64 - 1)) + 1) as i32;
                            println!("🔧 ID変換: ハッシュ後のID = {}", final_id);
                            final_id
                        })
                    }
                    Id::Number(n) => {
                        println!("🔧 ID変換: SurrealDB ID (Number) = {}", n);
                        *n as i32
                    }
                    Id::Array(a) => {
                        println!("🔧 ID変換: SurrealDB ID (Array) = {:?}", a);
                        // 配列の場合はハッシュ化
                        let mut hasher = DefaultHasher::new();
                        format!("{:?}", a).hash(&mut hasher);
                        let hash = hasher.finish();
                        ((hash % (i32::MAX as u64 - 1)) + 1) as i32
                    }
                    Id::Object(o) => {
                        println!("🔧 ID変換: SurrealDB ID (Object) = {:?}", o);
                        // オブジェクトの場合はハッシュ化
                        let mut hasher = DefaultHasher::new();
                        format!("{:?}", o).hash(&mut hasher);
                        let hash = hasher.finish();
                        ((hash % (i32::MAX as u64 - 1)) + 1) as i32
                    }
                    _ => {
                        println!("🔧 ID変換: SurrealDB ID (Unknown)");
                        1
                    }
                }
            })
            .unwrap_or(1); // IDが無い場合のデフォルト値

        println!("🔧 ID変換: 最終的なID = {}", id);
        Todo::reconstruct(id, record.title, record.completed)
    }
}

/// SurrealDB実装のTodoリポジトリ
use tokio::sync::Mutex;

pub struct TodoRepositoryImpl {
    db: DbClient,
    // async mutex for Send future
    next_id: Mutex<i32>,
}

impl TodoRepositoryImpl {
    pub fn new(db: DbClient) -> Self {
        Self {
            db,
            next_id: Mutex::new(0),
        }
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
        println!("🔍 find_by_id: IDで検索しています: todos:{}", id);

        let record: Option<TodoRecord> = self
            .db
            .select(("todos", id.to_string()))
            .await
            .map_err(|e| format!("データベースエラー: {}", e))?;

        println!("🔍 find_by_id: 検索結果: {:?}", record);
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

        // ID自動生成でレコードを作成
        let created: Option<TodoRecord> = self
            .db
            .create("todos")
            .content(new_todo)
            .await
            .map_err(|e| format!("データベースエラー: {}", e))?;

        let created = created.ok_or_else(|| "作成に失敗しました".to_string())?;
        println!("✅ save: 作成されたレコード: {:?}", created);
        let todo_entity = created.into();
        println!("✅ save: Todoエンティティ: {:?}", todo_entity);
        Ok(todo_entity)
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

use async_trait::async_trait;
use super::entity::Todo;

/// Todoリポジトリトレイト
/// インフラストラクチャ層で実装される永続化の抽象インターフェース
#[async_trait]
pub trait TodoRepository: Send + Sync {
    /// すべてのTodoを取得
    async fn find_all(&self) -> Result<Vec<Todo>, String>;
    
    /// IDでTodoを取得
    async fn find_by_id(&self, id: i32) -> Result<Option<Todo>, String>;
    
    /// Todoを保存（作成）
    async fn save(&self, todo: &Todo) -> Result<Todo, String>;
    
    /// Todoを更新
    async fn update(&self, todo: &Todo) -> Result<Todo, String>;
    
    /// Todoを削除
    async fn delete(&self, id: i32) -> Result<(), String>;
}

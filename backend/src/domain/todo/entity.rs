use serde::{Deserialize, Serialize};

/// Todoドメインエンティティ
/// ビジネスロジックとドメイン知識をカプセル化
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Todo {
    id: Option<i32>,
    title: String,
    completed: bool,
}

impl Todo {
    /// 新しいTodoを作成（永続化前、IDなし）
    pub fn new(title: String) -> Result<Self, String> {
        if title.trim().is_empty() {
            return Err("タイトルは空にできません".to_string());
        }
        
        Ok(Self {
            id: None,
            title: title.trim().to_string(),
            completed: false,
        })
    }

    /// 既存のTodoを再構築（永続化済み、ID付き）
    pub fn reconstruct(id: i32, title: String, completed: bool) -> Self {
        Self {
            id: Some(id),
            title,
            completed,
        }
    }

    /// IDを取得
    pub fn id(&self) -> Option<i32> {
        self.id
    }

    /// タイトルを取得
    pub fn title(&self) -> &str {
        &self.title
    }

    /// 完了状態を取得
    pub fn is_completed(&self) -> bool {
        self.completed
    }

    /// タイトルを変更
    pub fn change_title(&mut self, new_title: String) -> Result<(), String> {
        if new_title.trim().is_empty() {
            return Err("タイトルは空にできません".to_string());
        }
        self.title = new_title.trim().to_string();
        Ok(())
    }

    /// Todoを完了にする
    pub fn complete(&mut self) {
        self.completed = true;
    }

    /// Todoを未完了にする
    pub fn uncomplete(&mut self) {
        self.completed = false;
    }

    /// 完了状態をトグル
    pub fn toggle_completion(&mut self) {
        self.completed = !self.completed;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_todo() {
        let todo = Todo::new("テストタスク".to_string()).unwrap();
        assert_eq!(todo.title(), "テストタスク");
        assert!(!todo.is_completed());
        assert!(todo.id().is_none());
    }

    #[test]
    fn test_empty_title() {
        let result = Todo::new("  ".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_toggle_completion() {
        let mut todo = Todo::new("テスト".to_string()).unwrap();
        assert!(!todo.is_completed());
        
        todo.toggle_completion();
        assert!(todo.is_completed());
        
        todo.toggle_completion();
        assert!(!todo.is_completed());
    }
}

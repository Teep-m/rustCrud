use std::sync::Arc;
use actix_web::{web, HttpResponse, Result};
use crate::application::todo::{TodoService, CreateTodoDto, UpdateTodoDto};

/// 全Todoを取得
pub async fn get_todos(
    service: web::Data<Arc<TodoService>>
) -> Result<HttpResponse> {
    match service.get_all_todos().await {
        Ok(todos) => Ok(HttpResponse::Ok().json(todos)),
        Err(e) => Ok(HttpResponse::InternalServerError().body(e)),
    }
}

/// IDでTodoを取得
pub async fn get_todo(
    service: web::Data<Arc<TodoService>>,
    id: web::Path<i32>
) -> Result<HttpResponse> {
    match service.get_todo_by_id(id.into_inner()).await {
        Ok(todo) => Ok(HttpResponse::Ok().json(todo)),
        Err(e) => Ok(HttpResponse::NotFound().body(e)),
    }
}

/// Todoを作成
pub async fn create_todo(
    service: web::Data<Arc<TodoService>>,
    dto: web::Json<CreateTodoDto>
) -> Result<HttpResponse> {
    match service.create_todo(dto.into_inner()).await {
        Ok(todo) => Ok(HttpResponse::Created().json(todo)),
        Err(e) => Ok(HttpResponse::BadRequest().body(e)),
    }
}

/// Todoを更新
pub async fn update_todo(
    service: web::Data<Arc<TodoService>>,
    id: web::Path<i32>,
    dto: web::Json<UpdateTodoDto>
) -> Result<HttpResponse> {
    match service.update_todo(id.into_inner(), dto.into_inner()).await {
        Ok(todo) => Ok(HttpResponse::Ok().json(todo)),
        Err(e) => Ok(HttpResponse::NotFound().body(e)),
    }
}

/// Todoを削除
pub async fn delete_todo(
    service: web::Data<Arc<TodoService>>,
    id: web::Path<i32>
) -> Result<HttpResponse> {
    match service.delete_todo(id.into_inner()).await {
        Ok(_) => Ok(HttpResponse::Ok().body("削除しました")),
        Err(e) => Ok(HttpResponse::NotFound().body(e)),
    }
}

/// ルーティング設定
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/todos", web::get().to(get_todos))
            .route("/todos", web::post().to(create_todo))
            .route("/todos/{id}", web::get().to(get_todo))
            .route("/todos/{id}", web::put().to(update_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
    );
}

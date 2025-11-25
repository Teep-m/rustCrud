mod domain;
mod application;
mod infrastructure;
mod presentation;

use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

use infrastructure::{init_db, TodoRepositoryImpl};
use application::todo::TodoService;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 環境変数の読み込み
    dotenvy::dotenv().ok();

    println!("🚀 サーバーを起動中...");

    // データベース初期化
    let pool = init_db().await;
    println!("✅ データベース接続完了");

    // リポジトリ層の初期化
    let todo_repository = Arc::new(TodoRepositoryImpl::new(pool));

    // アプリケーション層（サービス）の初期化
    let todo_service = Arc::new(TodoService::new(todo_repository));

    println!("🌐 サーバーを http://0.0.0.0:8000 で起動します");

    // HTTPサーバーの起動
    HttpServer::new(move || {
        // CORS設定
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(todo_service.clone()))
            .configure(presentation::config)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}

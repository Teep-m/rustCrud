mod application;
mod domain;
mod infrastructure;
mod presentation;

use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use std::sync::Arc;

use application::todo::TodoService;
use infrastructure::{init_db, TodoRepositoryImpl};

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

    // ポート番号を環境変数から取得、デフォルトは8080
    let port = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8080);

    println!("🌐 サーバーを http://0.0.0.0:{} で起動します", port);

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
    .bind(("0.0.0.0", port))?
    .run()
    .await
}

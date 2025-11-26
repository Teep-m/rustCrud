use std::env;
use std::time::Duration;
use surrealdb::engine::remote::http::{Client, Http};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tokio::time::sleep;

pub type DbClient = Surreal<Client>;

/// データベース接続を初期化  
pub async fn init_db() -> DbClient {
    let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "http://127.0.0.1:8000".to_string());

    // http:// または https:// スキームを削除
    let db_host = db_url
        .strip_prefix("http://")
        .or_else(|| db_url.strip_prefix("https://"))
        .unwrap_or(&db_url);

    let db_user = env::var("DATABASE_USER").unwrap_or_else(|_| "root".to_string());

    let db_pass = env::var("DATABASE_PASS").unwrap_or_else(|_| "root".to_string());

    let db_ns = env::var("DATABASE_NS").unwrap_or_else(|_| "app".to_string());

    let db_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "todos".to_string());

    println!("📡 SurrealDB接続中: {}", db_host);

    // リトライロジック
    let mut retries = 0;
    let max_retries = 10;

    let db = loop {
        match Surreal::new::<Http>(db_host).await {
            Ok(db) => {
                println!("✅ SurrealDB接続成功");
                break db;
            }
            Err(e) => {
                retries += 1;
                if retries >= max_retries {
                    panic!(
                        "Failed to connect to SurrealDB after {} retries: {}",
                        max_retries, e
                    );
                }
                println!(
                    "⚠️  接続失敗 ({}/{}): {}. 再試行します...",
                    retries, max_retries, e
                );
                sleep(Duration::from_secs(2)).await;
            }
        }
    };

    println!("🔐 認証中...");

    // 認証
    db.signin(Root {
        username: &db_user,
        password: &db_pass,
    })
    .await
    .expect("Failed to sign in");

    println!("🗂️  Namespace/Database選択中: {}/{}", db_ns, db_name);

    // Namespace と Database を使用
    db.use_ns(&db_ns)
        .use_db(&db_name)
        .await
        .expect("Failed to use namespace and database");

    // スキーマ初期化
    init_schema(&db).await;

    db
}

/// スキーマとテーブルを初期化
async fn init_schema(db: &DbClient) {
    println!("📋 スキーマ初期化中...");

    // Todoテーブルの定義
    let _result = db
        .query(
            "
            DEFINE TABLE IF NOT EXISTS todos SCHEMAFULL;
            DEFINE FIELD IF NOT EXISTS title ON TABLE todos TYPE string;
            DEFINE FIELD IF NOT EXISTS completed ON TABLE todos TYPE bool DEFAULT false;
            DEFINE FIELD IF NOT EXISTS created_at ON TABLE todos TYPE datetime DEFAULT time::now();
            ",
        )
        .await
        .expect("Failed to initialize schema");

    println!("✅ スキーマ初期化完了");
}

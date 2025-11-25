# Rust CRUD Application

Rust (Actix-Web + Leptos) + SurrealDB + Nginx を使用したCRUDアプリケーション。
DDDアーキテクチャを採用しています。

## 技術スタック
- **Backend**: Rust (Actix-Web 4.x)
- **Frontend**: Rust (Leptos)
- **Database**: SurrealDB 2.x (次世代マルチモデルデータベース)
- **Reverse Proxy**: Nginx
- **Containerization**: Docker & Docker Compose

## SurrealDBについて
SurrealDBは、SQL、GraphQL、リアルタイム、ドキュメント指向、グラフ、ベクトル検索など、複数のデータモデルをサポートする次世代データベースです。

### 主な特徴
- マルチモデル: ドキュメント、グラフ、リレーショナル、ベクトル
- リアルタイムサブスクリプション
- スキーマレスまたはスキーマフル
- 組み込みの認証と権限管理
- SurrealQL (拡張SQL)

## アーキテクチャ
DDDに基づいた4層アーキテクチャ:
- **Domain層**: ビジネスロジック、エンティティ
- **Application層**: ユースケース、サービス
- **Infrastructure層**: データベース、リポジトリ実装
- **Presentation層**: API、HTTPハンドラー

詳細は `backend/DDD_ARCHITECTURE.md` を参照してください。

## 前提条件
- Docker & Docker Compose

## クイックスタート

```bash
make up
```

アプリケーションにアクセス:
- **Frontend**: http://localhost
- **Backend API**: http://localhost/api/todos
- **SurrealDB Web UI**: http://localhost:8080

## 主要コマンド

| コマンド | 説明 |
| --- | --- |
| `make up` | コンテナをビルドして起動 |
| `make down` | コンテナを停止・削除 |
| `make logs` | ログを表示 |
| `make clean-db` | DBデータを削除してリセット |

## API エンドポイント

### Todos
- `GET /api/todos` - 全Todoを取得
- `GET /api/todos/{id}` - 特定のTodoを取得
- `POST /api/todos` - Todoを作成
- `PUT /api/todos/{id}` - Todoを更新
- `DELETE /api/todos/{id}` - Todoを削除

### リクエスト例

```bash
# 一覧取得
curl http://localhost/api/todos

# 作成
curl -X POST http://localhost/api/todos \
  -H "Content-Type: application/json" \
  -d '{"title":"新しいタスク"}'

# 更新
curl -X PUT http://localhost/api/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"completed":true}'

# 削除
curl -X DELETE http://localhost/api/todos/1
```

## 開発

### SurrealDBの直接操作

```bash
# SurrealDBコンテナに接続
docker exec -it webapp-db-1 /surreal sql --conn http://localhost:8000 --user root --pass root --ns app --db todos

# 全Todoを表示
SELECT * FROM todos;

# Todoを作成
CREATE todos SET title = "Test", completed = false;
```

### GCP Compute Engine へのデプロイ

1. Compute Engine インスタンスを作成
2. Docker と Docker Compose をインストール
3. リポジトリをクローン
4. `make up` を実行
5. ファイアウォールで HTTP (80) を許可

## プロジェクト構成

```
.
├── backend/              # Actix-Web API サーバー
│   ├── src/
│   │   ├── domain/           # ドメイン層
│   │   │   └── todo/
│   │   ├── application/      # アプリケーション層
│   │   │   └── todo/
│   │   ├── infrastructure/   # インフラ層
│   │   │   └── persistence/
│   │   └── presentation/     # プレゼンテーション層
│   │       └── api/
├── frontend/             # Leptos WebAssembly アプリ
│   ├── src/
│   │   ├── lib.rs            # メインコンポーネント
│   │   └── main.rs           # エントリーポイント
│   └── index.html            # HTMLテンプレート
├── docker-compose.yml        # Docker構成
├── Dockerfile.backend        # Backendイメージ
├── Dockerfile.frontend       # Frontendイメージ
├── nginx.conf                # Nginx設定
└── Makefile                  # 便利コマンド
```

## バージョン情報
- Actix-Web: 4.4
- SurrealDB: 1.5
- Rust: Latest

## ライセンス
MIT

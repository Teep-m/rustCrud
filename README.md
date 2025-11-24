# Rust CRUD Application

Rust (Rocket + Leptos) + PostgreSQL + Nginx を使用したCRUDアプリケーション。

## 技術スタック
- **Backend**: Rust (Rocket)
- **Frontend**: Rust (Leptos)
- **Database**: PostgreSQL
- **Reverse Proxy**: Nginx
- **Containerization**: Docker

## 前提条件
- Docker & Docker Compose

## クイックスタート

```bash
make up
```

アプリケーションにアクセス:
- Frontend: http://localhost
- Backend API: http://localhost/api/todos

## 主要コマンド

| コマンド | 説明 |
| --- | --- |
| `make up` | コンテナをビルドして起動 |
| `make down` | コンテナを停止・削除 |
| `make logs` | ログを表示 |
| `make clean-db` | DBデータを削除してリセット |
| `make iseed TABLE=xxx` | 指定テーブルからSeeder SQLを生成 |
| `make seed FILE=xxx.sql` | 指定したSeederファイルをDBに実行 |

## 開発

### Seederの使い方

1. **データベースのデータからSeederを生成**:
   ```bash
   make iseed TABLE=todos
   ```
   → `backend/migrations/seed_todos.sql` が生成されます

2. **Seederを実行**:
   ```bash
   make seed FILE=seed_todos.sql
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
├── backend/           # Rocket API サーバー
│   ├── src/
│   │   ├── bin/
│   │   │   └── iseed.rs    # Seeder生成ツール
│   │   ├── main.rs         # エントリーポイント
│   │   ├── routes.rs       # APIルート
│   │   ├── models.rs       # データモデル
│   │   └── db.rs           # DB接続
│   └── migrations/         # SQLマイグレーション
├── frontend/          # Leptos WebAssembly アプリ
│   ├── src/
│   │   ├── lib.rs          # メインコンポーネント
│   │   └── main.rs         # エントリーポイント
│   └── index.html          # HTMLテンプレート
├── docker-compose.yml      # Docker構成
├── Dockerfile.backend      # Backendイメージ
├── Dockerfile.frontend     # Frontendイメージ
├── nginx.conf              # Nginx設定
└── Makefile                # 便利コマンド
```

## ライセンス

MIT

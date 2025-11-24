# Rust Rocket + Leptos + Postgres + Docker

このプロジェクトは、以下の技術スタックを使用したCRUDアプリケーションです。

- **Backend**: Rust (Rocket)
- **Frontend**: Rust (Leptos)
- **Database**: PostgreSQL
- **Reverse Proxy**: Nginx
- **Containerization**: Docker

## 前提条件
- Docker & Docker Compose がインストールされていること

## 実行方法

Makefileを用意していますので、以下のコマンドで簡単に操作できます。

1. **アプリケーションの起動**
   ```bash
   make up
   ```
   これだけで、ビルド、コンテナ起動、DBマイグレーション、初期データ（Seeder）の投入が自動的に行われます。

2. **アプリケーションへのアクセス**
   - フロントエンド: [http://localhost](http://localhost)
   - バックエンドAPI: [http://localhost/api/todos](http://localhost/api/todos)

3. **停止**
   ```bash
   make down
   ```

## 開発・構成

- **Backend**: `backend/` ディレクトリ。ポート8000で動作します。
- **Frontend**: `frontend/` ディレクトリ。WebAssemblyにコンパイルされます。
- **Database**: PostgreSQL。データはDockerボリュームに永続化されます。

### データベースについて
データベースのマイグレーションとSeeder（初期データ投入）は、バックエンドサーバーの起動時に**自動的に実行**されます。
- マイグレーションファイル: `backend/migrations`

データをリセットして最初からやり直したい場合は、以下のコマンドを実行してください。
```bash
make clean-db
```

### Seederの逆生成 (iseed)
既存のデータベースのテーブルデータから、Seeder（SQLファイル）を生成することができます。
Laravelの `iseed` のように、`INSERT ... ON CONFLICT DO UPDATE` 形式で出力されるため、データの差分更新が可能です。

**使い方:**
1. アプリケーションを起動しておきます (`make up`)。
2. 以下のコマンドを実行します（例: `todos` テーブルの場合）。
   ```bash
   make iseed TABLE=todos
   ```
3. `backend/migrations/seed_todos.sql` にSQLファイルが生成（または上書き）されます。

## GCP Compute Engine へのデプロイ

1. Compute Engine インスタンスを作成（例: e2-medium, Ubuntu）。
2. インスタンスに Docker と Docker Compose をインストール。
3. このリポジトリをクローン（またはファイルを転送）。
4. `make up` を実行。
5. GCPのファイアウォール設定で HTTP (80) トラフィックを許可。

## コマンド一覧 (Makefile)

| コマンド | 説明 |
| --- | --- |
| `make up` | コンテナをビルドして起動（バックグラウンド） |
| `make down` | コンテナを停止・削除 |
| `make build` | コンテナを再ビルド |
| `make logs` | ログを表示 |
| `make clean-db` | DBデータを削除してリセット（次回起動時に初期化） |
| `make iseed TABLE=xxx` | 指定テーブルからSeeder SQLを生成し、ホスト側に保存 |
| `make seed FILE=xxx.sql` | 指定したSeederファイルをDBに実行 |

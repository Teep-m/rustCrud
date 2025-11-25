# DDD (Domain-Driven Design) アーキテクチャ

このbackendは、DDDの原則に従って実装されています。

## ディレクトリ構成

```
src/
├── domain/              # ドメイン層
│   └── todo/
│       ├── entity.rs        # Todoエンティティ（ビジネスロジック）
│       └── repository.rs    # リポジトリトレイト（抽象）
│
├── application/         # アプリケーション層
│   └── todo/
│       ├── dto.rs           # Data Transfer Object
│       └── service.rs       # ユースケース実装
│
├── infrastructure/      # インフラストラクチャ層
│   └── persistence/
│       ├── database.rs      # DB接続管理
│       └── todo_repository.rs # リポジトリ実装
│
├── presentation/        # プレゼンテーション層
│   └── api/
│       └── todo_handler.rs  # APIハンドラー
│
└── main.rs              # エントリーポイント
```

## 各層の責務

### Domain層（ドメイン層）
- ビジネスロジックの中核
- エンティティとリポジトリインターフェースを定義
- 他の層に依存しない

### Application層（アプリケーション層）
- ユースケースの実装
- ドメイン層のオーケストレーション
- トランザクション境界

### Infrastructure層（インフラストラクチャ層）
- 外部リソースへのアクセス実装
- データベース、外部API等
- ドメイン層のインターフェースを実装

### Presentation層（プレゼンテーション層）
- HTTPリクエスト/レスポンスの処理
- ルーティング
- 入力検証

## 依存関係の方向

```
Presentation → Application → Domain ← Infrastructure
```

- すべての層がDomain層に依存
- Infrastructure層はDomain層のインターフェースを実装
- Domain層は他の層に依存しない（依存性逆転の原則）

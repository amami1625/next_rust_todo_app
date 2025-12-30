# アーキテクチャドキュメント

## システム全体図

```mermaid
graph TB
    subgraph "クライアント"
        Browser[Web ブラウザ]
    end

    subgraph "フロントエンド (未実装)"
        NextJS[Next.js 15<br/>App Router]
    end

    subgraph "バックエンド"
        API[Axum Web Server<br/>:8080]
        Router[Router<br/>ルーティング]
        Handlers[Handlers<br/>ビジネスロジック]
        Models[Models<br/>データ構造]
    end

    subgraph "データベース"
        PG[(PostgreSQL<br/>:5432)]
    end

    Browser -->|HTTP Request| NextJS
    NextJS -->|REST API| API
    API --> Router
    Router --> Handlers
    Handlers --> Models
    Handlers -->|SQL Query| PG
    PG -->|Result| Handlers
    Handlers -->|JSON Response| API
```

## バックエンドの詳細アーキテクチャ

```mermaid
graph LR
    subgraph "エントリーポイント"
        Main[main.rs<br/>アプリ起動]
    end

    subgraph "設定・初期化"
        Config[config.rs<br/>環境変数]
        DB[db/mod.rs<br/>接続プール]
    end

    subgraph "ルーティング"
        Routes[routes.rs<br/>URL定義]
    end

    subgraph "ハンドラー層"
        TodoHandlers[handlers/todos.rs<br/>CRUD処理]
    end

    subgraph "データ層"
        TodoModel[models/todo.rs<br/>Todo構造体]
        Migration[migrations/<br/>スキーマ定義]
    end

    subgraph "データベース"
        PostgreSQL[(PostgreSQL)]
    end

    Main --> Config
    Main --> DB
    Main --> Routes
    Routes --> TodoHandlers
    TodoHandlers --> TodoModel
    TodoHandlers --> PostgreSQL
    DB --> PostgreSQL
    Migration -.->|初回実行| PostgreSQL
```

## データフロー

### リクエストの流れ（例: Todo 作成）

```mermaid
sequenceDiagram
    participant C as クライアント
    participant A as Axum Server
    participant R as Router
    participant H as create_todo Handler
    participant P as PgPool (State)
    participant D as PostgreSQL

    C->>A: POST /api/todos<br/>{title, description}
    A->>R: リクエストをルーティング
    R->>H: create_todo() 呼び出し
    Note over H: Json(payload) で<br/>CreateTodo に変換
    H->>P: State(pool) から接続取得
    P->>D: INSERT INTO todos...
    D->>P: 作成された Todo を返す
    P->>H: Todo データ
    Note over H: Json(todo) に変換
    H->>A: 201 Created + JSON
    A->>C: レスポンス返却
```

### 全 CRUD 操作のフロー

```mermaid
graph TD
    Client[クライアント]

    Client -->|POST /api/todos| Create[create_todo]
    Client -->|GET /api/todos| GetAll[get_todos]
    Client -->|GET /api/todos/:id| GetOne[get_todo]
    Client -->|PUT /api/todos/:id| Update[update_todo]
    Client -->|DELETE /api/todos/:id| Delete[delete_todo]

    Create -->|INSERT| DB[(PostgreSQL)]
    GetAll -->|SELECT *| DB
    GetOne -->|SELECT WHERE id| DB
    Update -->|UPDATE WHERE id| DB
    Delete -->|DELETE WHERE id| DB

    DB -->|Todo| Create
    DB -->|Vec<Todo>| GetAll
    DB -->|Todo| GetOne
    DB -->|Todo| Update
    DB -->|204 No Content| Delete
```

## モジュール構成

```mermaid
graph TB
    subgraph "src/"
        Main[main.rs<br/>エントリーポイント]

        subgraph "設定"
            Config[config.rs<br/>Config構造体]
        end

        subgraph "データベース"
            DBMod[db/mod.rs<br/>接続・マイグレーション]
        end

        subgraph "ルーティング"
            Routes[routes.rs<br/>create_router]
        end

        subgraph "models/"
            ModelsMod[models/mod.rs]
            Todo[models/todo.rs<br/>Todo, CreateTodo, UpdateTodo]
        end

        subgraph "handlers/"
            HandlersMod[handlers/mod.rs]
            TodoHandlers[handlers/todos.rs<br/>5つのCRUD関数]
        end
    end

    Main --> Config
    Main --> DBMod
    Main --> Routes
    Main --> ModelsMod
    Main --> HandlersMod

    ModelsMod --> Todo
    HandlersMod --> TodoHandlers

    Routes --> TodoHandlers
    TodoHandlers --> Todo
```

## データモデル

```mermaid
erDiagram
    TODOS {
        uuid id PK "主キー"
        text title "タイトル（必須）"
        text description "説明（任意）"
        boolean completed "完了状態"
        timestamptz created_at "作成日時"
        timestamptz updated_at "更新日時"
    }
```

## API エンドポイント

| メソッド | パス | ハンドラー | 説明 |
|---------|------|-----------|------|
| GET | `/api/todos` | `get_todos` | 全 Todo を取得 |
| GET | `/api/todos/:id` | `get_todo` | 特定 Todo を取得 |
| POST | `/api/todos` | `create_todo` | 新規 Todo を作成 |
| PUT | `/api/todos/:id` | `update_todo` | Todo を更新 |
| DELETE | `/api/todos/:id` | `delete_todo` | Todo を削除 |

## 技術スタック詳細

### Backend
- **言語**: Rust 2021 Edition
- **Web Framework**: Axum 0.7
- **非同期ランタイム**: Tokio 1.x
- **データベースドライバ**: SQLx 0.7
- **シリアライゼーション**: Serde 1.0
- **ロギング**: tracing + tracing-subscriber
- **エラーハンドリング**: anyhow + thiserror

### Database
- **RDBMS**: PostgreSQL 16
- **マイグレーション**: SQLx migrations
- **接続プール**: SQLx PgPool (max 5 connections)

### Infrastructure
- **コンテナ**: Docker + Docker Compose
- **開発環境**: Docker による一括管理

## 主要な設計パターン

### 1. レイヤードアーキテクチャ

```
┌─────────────────────┐
│   Routes (ルーティング)  │
├─────────────────────┤
│  Handlers (ハンドラー)   │
├─────────────────────┤
│  Models (データモデル)   │
├─────────────────────┤
│  Database (永続化)   │
└─────────────────────┘
```

### 2. 依存性注入（State パターン）

```rust
// Router に State を登録
Router::new()
    .route("/api/todos", get(get_todos))
    .with_state(pool)  // ← pool を注入

// Handler で State を受け取る
async fn get_todos(State(pool): State<PgPool>) {
    // pool が自動的に渡される
}
```

### 3. エクストラクターパターン

Axum のエクストラクターを活用：
- `State<T>`: アプリケーション状態
- `Json<T>`: リクエスト/レスポンスボディ
- `Path<T>`: URL パラメータ

## セキュリティ考慮事項

1. **SQL インジェクション対策**: SQLx のプレースホルダー (`$1, $2`) を使用
2. **型安全性**: Rust の型システムでコンパイル時チェック
3. **エラーハンドリング**: すべてのエラーを適切に処理し、ログに記録

## パフォーマンス最適化

1. **接続プール**: データベース接続を再利用（最大5接続）
2. **非同期処理**: Tokio による効率的な並行処理
3. **インデックス**: `completed` と `created_at` にインデックスを設定

## 今後の拡張予定

- [ ] フロントエンド（Next.js）の実装
- [ ] 認証・認可機能
- [ ] ページネーション
- [ ] フィルタリング・検索機能
- [ ] Todo のカテゴリー・タグ機能

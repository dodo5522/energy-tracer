# Energy Tracer Backend

`axum + utoipa + SeaORM` を使ったバックエンド API プロジェクトです。  
Rust バックエンドのルートは `backend/` です。

## アーキテクチャ

現在は Rust API とローカル PostgreSQL、TanStack Start の認証デモを開発しています。将来は個人利用・低コストを前提に、フロントエンドとバックエンドを ECS Fargate の 1 タスクに配置し、認証・設定を Neon PostgreSQL、発電の時系列データを DynamoDB で管理する構想です。これらの AWS 構成と認証・認可は未実装です。

詳細は [アーキテクチャ設計](docs/02_architecture.md)、[認証・デバイス設計](docs/05_auth_device_design.md)、[Better Auth チュートリアル](docs/tutorials/better-auth-postgresql.md) を参照してください。

## ディレクトリ構成

- `backend/` Rust バックエンド（Cargo ワークスペース）
- `backend/crates/presentation` HTTP ルーティング・DTO・OpenAPI
- `backend/crates/use-case` アプリケーションサービス
- `backend/crates/domain` ドメインモデル・リポジトリ境界
- `backend/crates/infra` インフラストラクチャ実装（DBはSeaORM）
- `backend/crates/infra-db-migration` マイグレーション
- `docker/` 開発用コンテナ定義
- `init/` DB 初期化 SQL / 設定

## 前提環境

- Rust（`backend/rust-toolchain.toml`）
- Docker / Docker Compose

## 起動方法

### 1. Docker Compose で起動（DB + Backend）

フロントエンドの Compose 設定は作業途中です。DB とバックエンドのみ起動する場合はサービスを指定します。

```bash
docker compose up -d db backend
```

- DB: `127.0.0.1:5432`
- API: `127.0.0.1:8000`

### 2. ローカルでバックエンドのみ起動

DB を先に起動したうえで実行します。

```bash
cd backend
cargo run
```

`BIND_ADDR` / `BIND_PORT` はリポジトリルートの `backend/.env` で設定します。

## テスト・チェック

```bash
cd backend
cargo test
cargo check
```

## マイグレーション

```bash
cd backend
cargo run -p infra-db-migration -- status
cargo run -p infra-db-migration -- up
```

## API ドキュメント

- Swagger UI: `http://127.0.0.1:8000/docs/swagger`
- ReDoc: `http://127.0.0.1:8000/docs/redoc`
- OpenAPI JSON: `http://127.0.0.1:8000/openapi.json`

## 主要エンドポイント

- `GET /health/` ヘルスチェック（`204 No Content`）
- `POST /generation/history`
- `GET /generation/history/{id}`
- `POST /generation/label`
- `GET /generation/label`
- `POST /generation/sub_system`
- `GET /generation/sub_system`
- `POST /generation/unit`
- `GET /generation/unit`

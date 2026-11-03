# Energy Tracer Backend

`axum + utoipa + SeaORM` を使ったバックエンド API プロジェクトです。  
Rust バックエンドのルートは `backend/` です。

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

```bash
docker compose up -d
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

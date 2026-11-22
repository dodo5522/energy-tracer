# Energy Tracer

自家発電のデータを記録・可視化するアプリケーションのモノレポです。Rust / axum・SeaORM によるバックエンド、TanStack Start によるフロントエンド、インフラ構成を管理します。

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
- `frontend/` TanStack Start フロントエンド
- `infra/` インフラ構成（Terraform、作成途中）
- `docs/` 要求仕様・設計・チュートリアル
- `.devcontainer/` 開発コンテナ設定
- `docker/` コンテナ定義
- `init/` DB 初期化 SQL / 設定

## 前提環境

- Docker / Docker Compose
- コンテナ外で実行する場合: Rust（`backend/rust-toolchain.toml`）、Node.js / pnpm（`frontend/package.json`）

## 起動方法

### 1. Docker Compose で起動（DB + Backend）

開発用の backend と frontend は共通イメージ `base-dev:latest` を使います。初回およびベース定義や `mise.toml` の変更時に、リポジトリルートでビルドします。

```bash
docker build -t base-dev:latest -f docker/base-dev/Dockerfile .
```

DB とバックエンドを起動する場合:

```bash
docker compose up -d db backend
```

- DB: `127.0.0.1:5432`
- API: `127.0.0.1:8000`

### 2. フロントエンドの開発

Compose の `frontend` は作業途中です。DB の準備後、起動時に依存関係のインストール、Better Auth のマイグレーション、開発サーバーの起動を順に実行する構成です。環境設定を確認したうえで `docker compose up -d frontend` を試せます。フロントエンドから DB へ接続する場合、ホスト上で動かすと `DB_HOST=127.0.0.1`、Compose のコンテナ内では `DB_HOST=db` を使用します。ホストからブラウザでアクセスする URL は `http://localhost:3000` です。

マイグレーション用・実行用の環境変数やローカルでの手順は [Better Auth チュートリアル](docs/tutorials/better-auth-postgresql.md) を参照してください。

コンテナ外でフロントエンドを起動する場合は、DB と環境変数を準備してから実行します。

```bash
cd frontend
pnpm install
pnpm dev
```

### 3. ローカルでバックエンドのみ起動

DB を先に起動したうえで実行します。

```bash
cd backend
cargo run
```

`BIND_ADDR` / `BIND_PORT` はリポジトリルートの `backend/.env` で設定します。

## テスト・チェック

バックエンド:

```bash
cd backend
cargo test
cargo check
```

フロントエンド:

```bash
cd frontend
pnpm test
pnpm check
```

## マイグレーション

バックエンドの SeaORM マイグレーション:

```bash
cd backend
cargo run -p infra-db-migration -- status
cargo run -p infra-db-migration -- up
```

Better Auth のマイグレーションは [チュートリアル](docs/tutorials/better-auth-postgresql.md) を参照してください。

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

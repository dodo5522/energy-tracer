# 開発ガイドライン

最終更新日: 2026-02-25

## 1. 目的

- 本ドキュメントは、日常開発で参照する実務ルール（コーディング、テスト、コミット、設定管理）を整理する。

## 2. 適用範囲

- `backend/`（Rust バックエンド）
- `frontend/`（TanStack Start フロントエンド）
- リポジトリ全体のドキュメント・コミット運用

## 3. 共通ルール

### 3.1 言語方針

- ドキュメントは日本語で記述する。
- コードコメントは日本語で記述する。
- 変数名、関数名、構造体名、型名などの識別子は英語で記述する。
- 自動生成コード（例: SeaORM Entity）は生成物の表記を優先し、無理に手修正しない。

### 3.2 ディレクトリ構成

- `init/` は初期化スクリプト群
- `docker/` は開発用コンテナと本番用コンテナ
    - `backend` は本番用バックエンド
    - `backend-dev` は開発用バックエンド

## 4. backend 開発ガイドライン

### 4.1 採用技術

- 言語: Rust
- HTTP サーバ: axum
- 非同期ランタイム: tokio
- API ドキュメント: utoipa, utoipa-swagger-ui, utoipa-redoc
- ORM / DB: SeaORM
- DB マイグレーション: SeaORM migration

### 4.2 ディレクトリ構成

- `backend/`
    - `src/` は API サーバのエントリポイント
    - `crates/` はワークスペースのクレート群:
        - `crates/presentation/` HTTP ルーティングとリクエスト/レスポンスモデル
        - `crates/use-case/` アプリケーション/サービス層、プレゼンテーションとのインタフェース
        - `crates/domain/` ドメインとリポジトリのインターフェース
        - `crates/infra/` SeaORM のエンティティ、リポジトリ、DB 接続補助
        - `crates/infra-db-migration/` SeaORM のマイグレーション CLI とファイル群

### 4.3 リクエスト処理の流れ

対象例: 発電履歴の作成（`POST /generation/history`）

1. `presentation` で HTTP リクエストを受け取る
2. リクエスト DTO を use-case 入力 DTO へ変換
3. `use-case` が Unit of Work を開始
4. `infra` の Repository 実装が SeaORM で DB へ保存
5. 成功時 `commit` / 失敗時 `rollback`
6. `presentation` が HTTP レスポンスへ変換して返却

### 4.4 コーディング規約

- `cargo fmt` に従う（標準 Rust フォーマッタ）。
- 命名は Rust 慣例に従う。
    - 関数/モジュール: `snake_case`
    - 型/トレイト: `CamelCase`
- レイヤ境界を跨いだ責務漏れを避ける。
    - `presentation` に DB 詳細を持ち込まない
    - `domain` に HTTP/ORM 依存を持ち込まない

### 4.5 ビルド・テスト・開発コマンド

- `docker compose up -d` で PostgreSQL を起動し、マイグレーション、APIサーバを起動。
- `cd backend && cargo run` で API サーバを起動（`BIND_ADDR:BIND_PORT` にバインド）。
- `cd backend && cargo test` でワークスペース全体のテストを実行（存在する場合）。
- `cd backend && cargo run -p infra-db-migration -- status` でマイグレーションの状態確認。
- `cd backend && cargo run -p infra-db-migration -- up` で未適用マイグレーションを適用。

### 4.6 テスト方針

- `backend/` 配下で `cargo test` を実行する。
- テストは対象コードの近くに配置する（`crates/*/src/`）。
- DB 依存テストを追加する場合は以下をセットで記載する。
    - 必要な環境変数
    - DB 初期化手順
    - 実行コマンド

## 5. frontend 開発ガイドライン

### 5.1 使用技術

- フレームワーク: TanStack Start
- 言語: TypeScript + React
- package manager: `pnpm`
- 認証（現行デモ実装）: Better Auth
- 静的解析/整形: Biome
- テスト: Vitest

### 5.2 ディレクトリ構成

- `frontend/` はWebフロントエンドのルート
    - `public/`
    - `src/`
        - `components/atom/` 基本的な UI コンポーネント
        - `components/molecule/` 複数の atom を組み合わせたコンポーネント
        - `components/organism/` 複数の molecule を組み合わせ
        - `components/template/` ページレイアウトを定義
        - `components/page/` ルーティングされるページコンポーネント
        - `integrations/` 認証・API クライアントなどの基盤連携コード
        - `lib/` 汎用ユーティリティ関数
        - `hooks/` 共通カスタムフック
        - `routes/` ルーティング定義

### 5.3 コーディング規約

- 型安全性を優先し、`any` の常用を避ける。
- 命名は React / TypeScript 慣例に従う。
    - コンポーネント: `PascalCase`
    - 関数・変数: `camelCase`
    - カスタムフック: `useXxx`
- UI 層から直接 `fetch` を乱立させず、API クライアント層を分離する。
- 認証・データ取得などの基盤連携コードは `src/integrations/` / `src/lib/` にまとめる。

## 5.4 開発・確認コマンド

- `cd frontend && pnpm install` で依存関係をインストール。
- `cd frontend && pnpm run dev` で開発サーバを起動。
- `cd frontend && pnpm run build` で本番ビルドを作成。
- `cd frontend && pnpm run test` でテストを実行。
- frontend の package manager は `pnpm` に統一し、`npm` / `yarn` を混在させない。
- `pnpm-lock.yaml` を lockfile として管理する。
- Node / `pnpm` のバージョン番号は `AGENTS.md` に固定値を書かず、`frontend/package.json`（`engines` / `packageManager`）と
  `pnpm` 設定（例: `frontend/.npmrc`）を参照する。

## 5.5 テスト方針

- 単体テストを優先し、変換・バリデーション・ユーティリティを主対象とする。
- 重要画面には最小限の統合テストを追加し、SSR 初期表示と API 連携の回帰を検知する。
- 前提条件（環境変数など）は `frontend/` 配下の README に明記する。

## 6. コミット / PR ガイドライン

### 6.1 コミット

- スコープ付きプレフィックス形式を推奨する。
    - 例: `[infra] Add migration files`
    - 例: `[frontend] Add auth route guard skeleton`
- 1コミット1変更（または1レイヤ）を意識する。

### 6.2 Pull Request

- PR には以下を含める。
    - 変更概要
    - 実行したコマンド（例: `cargo test`, `pnpm run test`）
    - DB スキーマ変更の有無
    - 影響範囲（backend / frontend / docs）

## 7. 秘密情報の扱い

- 秘密情報はローカルの `../backend/.env` で管理する。
- リポジトリに含まれる `../backend/.env` は開発用サンプルとして扱う。
- 認証情報をログ・スクリーンショット・PR 説明へ貼らない。

## 8. 現時点の注意点（学習 / スパイク段階）

- frontend は TanStack Start（RC 含む）を前提としているため、破壊的変更の追従コストを許容する。
- 生成コードをすぐに大規模改変せず、先に構成理解と最小スケルトン化を優先する。
- フロントエンドの本番向け保護ページ導線は未実装のため、認証デモ実装と混同しない。

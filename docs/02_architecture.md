# アーキテクチャ設計

最終更新日: 2026-03-15

## 1. 目的

- 本ドキュメントは、システム構成とレイヤ責務を整理する。
- 現時点では学習・スパイク段階のため、実装済みの内容を優先して記載し、未実装部分は今後の拡張方針として分離する。

## 2. 全体構成

- Rust バックエンド API（`axum + utoipa + SeaORM`）
- TanStack Start ベースの Web フロントエンド（SSR 対応）
- frontend（TanStack Start）の API ルートで Better Auth を提供
- frontend は TanStack Start の server function を BFF として使い、Web 画面から Rust バックエンド API を仲介して呼び出す
- DB は PostgreSQL を想定
- コンテナ構成は未確定（ローカル開発は Docker Compose 、本番環境はECSを予定）

参考:

- TanStack Start Server Functions
    - https://tanstack.com/start/latest/docs/framework/react/guide/server-functions
- TanStack Start Execution Model
    - https://tanstack.com/start/latest/docs/framework/react/guide/execution-model

### 2.1 構成イメージ

1. 自宅デバイス / PC がバックエンド API に発電データを `POST`
2. バックエンドが DB に保存
3. ユーザーがフロントエンド Web 画面にアクセス
4. フロントエンドは認証（Better Auth）を使って可視化画面を提供する
5. 可視化画面からのデータ取得は browser から Rust API を直接呼ばず、TanStack Start の server function を経由して Rust
   バックエンド API を呼ぶ

### 2.2 BFF の位置づけ

- 本プロジェクトにおける BFF（Backend For Frontend）は、frontend サーバ内で動く TanStack Start の server function を指す。
- browser は frontend が公開する server function を呼び、その server function が Rust バックエンド API を呼ぶ。
- これにより、`BACKEND_BASE_URL` などのサーバ専用環境変数は browser に公開しない。
- 今回の採用理由は、SSR と server/client の責務分離を学習しやすくするためである。

参考:

- TanStack Start Environment Variables
    - https://tanstack.com/start/latest/docs/framework/react/guide/environment-variables

## 3. バックエンドアーキテクチャ（`backend/`）

### 3.1 レイヤ構成と責務

オニオンアーキテクチャを採用し、各層の責務を以下のように分類する。

- プレゼンテーション
    - HTTP ルーティング、リクエスト/レスポンス DTO、OpenAPI 公開
    - 入口レイヤとして use-case を呼び出す
- インフラストラクチャ
    - SeaORM モデル、Repository 実装、DB 接続、Unit of Work 実装
    - DB 依存の詳細を use-case 層から隠蔽
- インフラストラクチャ（DBマイグレーション）
    - スキーマ変更（マイグレーション）管理
- ユースケース
    - アプリケーションサービス
    - リポジトリ/Unit of Work のインターフェース定義
    - ドメインエンティティと入出力 DTO の橋渡し
- ドメイン
    - エンティティ、値オブジェクト（例: `Unit`）
    - ドメイン表現の中心

## 4. フロントエンドアーキテクチャ（`frontend/`）

### 4.1 レイヤ構成と責務

アトミックデザインを採用し、各層の責務を以下のように分類する。

- ATOMS
    - 基本的な UI コンポーネント（例: ボタン、入力フィールド、ラベル）
- MOLECULES
    - 複数の ATOMS を組み合わせたコンポーネント（例: フォーム、カード）
- ORGANISMS
    - 複数の MOLECULES を組み合わせたコンポーネント（例: ヘッダー、サイドバー）
- TEMPLATES
    - ページのレイアウトを定義するコンポーネント（例: ダッシュボードレイアウト）
- PAGES
    - ルーティングされるページコンポーネント（例: ログインページ、可視化ページ）

### 4.2 実行モデル

- `components/pages` / `components/*`
    - UI 表示とユーザー操作を担当する。
    - `fetch` やバックエンド URL を直接扱わない。
- `routes/`
    - ルーティング定義と route 単位の責務分離を担当する。
    - 必要に応じて page component に props を渡す。
- `integrations/`
    - 外部連携コードを置く。
    - Rust バックエンド API 連携は server function を公開する層として扱う。
    - browser から呼べる関数を export しても、実際のバックエンド通信は server function の handler 内で行う。
- server function
    - frontend の BFF として振る舞う。
    - `process.env.BACKEND_BASE_URL` などのサーバ専用環境変数を使って Rust バックエンド API を呼ぶ。
    - 共通の認証付与、リクエスト整形、レスポンス変換、エラー変換の集約先とする。

参考:

- TanStack Start Server Functions
    - https://tanstack.com/start/latest/docs/framework/react/guide/server-functions
- TanStack Start Environment Variables
    - https://tanstack.com/start/latest/docs/framework/react/guide/environment-variables

### 4.3 API 連携方針

- browser から Rust バックエンド API へ直接 `fetch` しない。
- `frontend/src/integrations/home-energy-manager/` 配下に、機能ごとの server function を配置する。
- 例として labels 取得は `frontend/src/integrations/home-energy-manager/generation/label/index.ts` の server function から
  Rust バックエンド API の `/generation/labels` を呼ぶ。
- page component は TanStack Query の `queryFn` などから、その server function をラップした関数を呼ぶ。
- backend のベース URL は `VITE_` 付き環境変数ではなく、frontend サーバの `BACKEND_BASE_URL` で管理する。

この方針により、以下を両立する。

- SSR 学習のための server/client 境界の明確化
- browser への不要な環境変数公開の回避
- 将来の JWT や認証ヘッダ付与の集約
- UI 層での `fetch` 乱立の回避

## 5. 今後の設計論点（未確定）

- 同一送信元の判定方法（`device_id` 等）とレート制御エラー仕様
- API 送信元認証（JWT）導入のタイミングと移行方針
- フロントエンドの保護ページ導線（未ログイン時リダイレクト含む）
- 可視化画面の API クライアント層の配置と責務

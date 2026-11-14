# Better Auth と PostgreSQL を TanStack Start に導入する

このチュートリアルは、実装者が既存の認証デモを PostgreSQL に接続し、登録・ログイン・ログアウトと保護ページを試すための手順です。まずローカル PostgreSQL を使い、その後 Neon に接続します。Rust API の認証や DynamoDB は対象外です。

## 完成イメージ

ブラウザは TanStack Start の `/api/auth/*` に HTTPS（ローカルでは HTTP）でアクセスします。サーバー側の Better Auth が PostgreSQL に接続し、ユーザーとセッションを管理します。ブラウザにはセッション Cookie が渡り、DB の接続情報は渡しません。

- `frontend/src/lib/auth.ts`: 通常実行用の Better Auth 設定（`auth_operator`）
- `frontend/src/lib/auth-migration.ts`: CLI マイグレーション用の Better Auth 設定（`migrator`）
- `frontend/src/routes/api/auth/$.ts`: 認証 API の入口
- `frontend/src/lib/auth-client.ts`: React から呼び出すクライアント
- `frontend/src/components/pages/LogIn.tsx`: 登録・ログイン画面
- `frontend/src/components/molecules/HeaderUser.tsx`: セッション表示・ログアウト
- `frontend/src/routes/config.tsx`: 保護対象の例

これらのファイルは既に存在します。現状 `auth.ts` には PostgreSQL 接続設定がありますが、`Configuration.tsx` の表示制御だけではサーバーや Rust API の認可にはなりません。既存の業務 API 取得は認証確認前に開始される点にも注意してください。

## 1. 前提と依存関係

Node と pnpm は `frontend/package.json` の指定を使います。以下、明記のないコマンドは `frontend/` で実行します。現在の package.json は `better-auth` と CLI パッケージ `auth` に `^1.7.25` を指定しています。実際にインストールされるバージョンは lockfile を確認してください。

```bash
cd frontend
pnpm install
pnpm list better-auth auth
```

`better-auth`、`pg`、`@types/pg`、CLI の `auth` は既に直接依存として登録されています。

## 2. ローカル DB と環境変数

リポジトリルートから PostgreSQL だけを起動します。

```bash
docker compose up -d db
docker compose exec db pg_isready -U postgres
```

Compose の初期化 SQL は `energy` DB を作成します。以下はホスト上でフロントエンドを起動する場合の**ローカル専用**の接続例です。DB が以前から存在する場合は接続先を確認してください。

`frontend/.env.local` のローカル接続設定を確認します。このファイルは現在 Git 管理されているため、実際のパスワードや秘密鍵をコミットしないでください。`BETTER_AUTH_SECRET` は次のコマンドで生成し、出力を手動で設定してください。

```bash
openssl rand -base64 32
```

```dotenv
DB_HOST=127.0.0.1
DB_PORT=5432
DB_NAME=energy
DB_AUTH_OPERATOR_NAME=auth_operator
DB_AUTH_OPERATOR_PASSWORD=ローカルのauth_operatorのパスワード
DB_MIGRATOR_NAME=migrator
DB_MIGRATOR_PASSWORD=ローカルのmigratorのパスワード
BETTER_AUTH_URL=http://localhost:3000
BETTER_AUTH_SECRET=ここに生成した秘密鍵を貼る
```

`localhost` と `127.0.0.1` はブラウザからは異なるオリジンです。アプリへのアクセスは `http://localhost:3000` に統一します。URL や秘密鍵を `VITE_` 付きの変数にせず、コミットやログへの出力もしないでください。`DB_HOST` の `127.0.0.1` はホストからの接続先であり、将来フロントエンドをコンテナで動かす場合は適切なホスト名に変更します。

## 3. Better Auth のサーバー設定

`frontend/src/lib/auth.ts` は既に `pg.Pool` を使い、`DB_HOST`、`DB_PORT`、`DB_NAME` と通常実行用の `DB_AUTH_OPERATOR_NAME`、`DB_AUTH_OPERATOR_PASSWORD` を読み取ります。`auth-migration.ts` は共通の接続先と、マイグレーション用の `DB_MIGRATOR_NAME`、`DB_MIGRATOR_PASSWORD` を使用します。`options: '-c search_path=auth'` により `auth` スキーマを接続先に指定します。現行実装は必須変数の検証や Neon 接続用 TLS の設定を明示していないため、導入時に確認してください。

`BETTER_AUTH_URL` と `BETTER_AUTH_SECRET` は Better Auth が環境変数から読み取ります。`tanstackStartCookies()` はプラグインを増やす場合も配列の最後に置きます。`auth.ts` はサーバー専用として扱い、React コンポーネントから直接 import しません。既存の `src/env.ts` は `import.meta.env` を使う一方、`auth.ts` は `process.env` を使います。CLI 実行時も環境変数が渡るようにします。

## 4. 認証テーブルをマイグレーションする

Better Auth は `user`、`session`、`account`、`verification` などを使います。これらは Better Auth が管理し、業務テーブルの SeaORM Migration とは分けます。現在の初期化 SQL は `energy` DB の `auth` スキーマと `migrator`、`auth_operator` ロールを作成します。既存 DB には初期化 SQL が自動で再適用されないため、スキーマと権限を確認してください。詳細は [認証・デバイス設計](../05_auth_device_design.md) を参照してください。

マイグレーションは `migrator`、通常のアプリ起動は `auth_operator` で行います。CLI に渡される `DB_MIGRATOR_NAME` と `DB_MIGRATOR_PASSWORD` を設定し、ホスト・ポート・DB 名が対象の開発 DB を指すことを確認してください。両設定は `process.env` を読みます。Better Auth CLI 1.7.25 は設定読み込み時に作業ディレクトリの `.env` と `.env.local` を対象にしますが、シェルに設定済みの環境変数も含めて意図した接続先・ロールになるか確認し、秘密情報は表示・共有しないでください。

インストール済みの `auth` CLI でマイグレーションします。`--config` は設定ファイルを明示します。

```bash
pnpm exec auth migrate --config src/lib/auth-migration.ts
```

`auth-migration.ts` と `auth.ts` は別の設定です。スキーマに影響する認証方式やプラグインを変更する際は両方の設定の整合性を確認してください。

このプロジェクトでは旧 CLI `@better-auth/cli` に代わり、`auth` パッケージを使用します。ライブラリと CLI は互換性のあるバージョンを使い、アップグレード時には両方の変更履歴と lockfile を確認してください。`pnpm dlx auth@latest` は最新版を別途取得するため、プロジェクトのインストール済み CLI とは異なるバージョンになり得ます。

`migrate` は DB を変更します。実行前に環境変数の接続先と CLI が表示する変更内容を確認してください。本番 DB ではなく、まずローカル DB で試します。作成結果はリポジトリルートから確認できます。

```bash
docker compose exec db psql -U postgres -d energy -c '\dt auth.*'
```

認証テーブルの有無だけを確認し、パスワードハッシュやセッショントークンの内容は表示・共有しないでください。プラグイン追加時などはスキーマ変更を再確認します。SQL をレビューして履歴管理したい場合は CLI の `generate` も選択肢です。生成と適用の運用を決めてから本番に進んでください。

### Docker Compose の検討状況

現在の `docker-compose.yml` はフロントエンド内でマイグレーション後に開発サーバーを起動する構成を検討中で、まだ完成・動作確認されていません。同一コンテナに両ロールの環境変数を渡すため、接続設定は分かれても migrator の認証情報は通常動作中のコンテナから見えます。マイグレーション専用サービスへの分離、成功後の起動順序と失敗時の停止を今後検討します。

## 5. サーバールートとクライアントを確認する

既存の `src/routes/api/auth/$.ts` は GET / POST を `auth.handler(request)` に渡します。`src/lib/auth-client.ts` の `createAuthClient()` は同一オリジンの認証 API に接続します。通常は新たに同じファイルを作る必要はありません。

既存の `LogIn.tsx` は以下の API を使用しています。実装を追い、フォーム入力とレスポンスのエラー表示の対応を確認してください。

```ts
await authClient.signUp.email({ name, email, password });
await authClient.signIn.email({ email, password });
await authClient.signOut();
const { data: session, isPending } = authClient.useSession();
```

`useSession()` は UI の状態表示に便利ですが、認可の根拠をブラウザに任せないでください。登録画面は学習用です。インターネットに公開する前に、誰でも登録できる状態をどう制限するか、メール検証やパスワード再設定をどう扱うかを決めます。

## 6. サーバー側でページを保護する

`src/lib/auth.functions.ts` を作り、リクエストの Cookie を含むヘッダーを Better Auth に渡します。

```ts
import { createServerFn } from '@tanstack/react-start';
import { getRequestHeaders } from '@tanstack/react-start/server';
import { auth } from '#/lib/auth';

export const getSession = createServerFn({ method: 'GET' }).handler(async () => {
  return auth.api.getSession({ headers: getRequestHeaders() });
});
```

`src/routes/config.tsx` のルートに `beforeLoad` を加え、未認証ならログインページへ移動します。既存の `component` は維持できます。

```tsx
import { createFileRoute, redirect } from '@tanstack/react-router';
import { Configuration } from '#/components/pages';
import { getSession } from '#/lib/auth.functions';

export const Route = createFileRoute('/config')({
  beforeLoad: async () => {
    const session = await getSession();
    if (!session) {
      throw redirect({ to: '/login', search: { redirect: '/config' } });
    }
    return { user: session.user };
  },
  component: () => <Configuration search={{ redirect: '/config' }} />,
});
```

これは画面への導線の保護です。**各サーバー関数や API は、それぞれの処理でセッションとアクセス権を検証する必要があります。** 現在の `Configuration.tsx` が呼ぶ Rust API はこのルートガードでは保護されません。API 認証を設計するまでは機密データを扱わないでください。また、既存のログイン後リダイレクトは先頭が `/` かだけを確認しています。外部へのリダイレクトを避けるため、許可する内部パスを明示する方式などに見直してください。

## 7. 動作確認

```bash
pnpm dev
```

1. `http://localhost:3000/login` で開発用ユーザーを登録する。
2. ヘッダーの表示がログイン状態に変わること、ページを再読み込みしても状態が維持されることを確認する。
3. ログアウト後、正しい情報で再ログインでき、誤ったパスワードでは失敗することを確認する。
4. 未ログインで `/config` に直接アクセスし、ログイン画面へ移動することを確認する。ログイン後に戻る動作も確認する。
5. 開発サーバーを再起動し、登録ユーザーが残ることを確認する。セッション有効期限や Cookie の状態によっては再ログインが必要な場合がある。
6. `pnpm exec tsc --noEmit`、`pnpm run check`、`pnpm run test`、`pnpm run build` を実行し、既存のエラーと今回の変更を切り分ける。

接続失敗時は DB の起動、ホスト・ポート・DB 名、ロールの権限、CLI が読み込む環境変数を確認します。Cookie の問題は URL のホスト・ポート、ブラウザの開発者ツール、サーバー側ログを確認します。秘密情報や Cookie の値は共有しないでください。

## 8. Neon に移す前に

Neon に開発用プロジェクトを作り、接続情報からホスト・ポート・DB 名・ユーザー・パスワードをそれぞれの `DB_*` 変数に設定します。現行の `pg.Pool` は TLS を明示していないため、Neon の要件に合わせた TLS 設定を追加・検証し、本番とは別の DB でマイグレーションを先に試します。ECS の実行環境から Neon に外向き通信できる必要があります。

ローカル初期化 SQL にある `migrator` と `auth_operator` の分離を Neon 上にも適用できるか確認し、`auth` スキーマと権限、実行時と CLI の接続先を準備します。Neon での手順は未検証です。Neon の停止からの復帰時の接続・再試行、無料枠、バックアップと復元、デプロイ時のマイグレーション手順を確認してください。`BETTER_AUTH_URL` は実際の HTTPS の公開 URL に変更し、秘密鍵は安全な仕組みで注入します。

## 参考

- [Better Auth: PostgreSQL](https://www.better-auth.com/docs/adapters/postgresql)
- [Better Auth: CLI](https://www.better-auth.com/docs/concepts/cli)
- [Better Auth: TanStack Start](https://www.better-auth.com/docs/integrations/tanstack)
- [Better Auth: Email & Password](https://www.better-auth.com/docs/authentication/email-password)
- [Neon: 接続](https://neon.com/docs/connect/connect-from-any-app)

import { betterAuth } from 'better-auth';
import { Pool } from 'pg';
import { tanstackStartCookies } from 'better-auth/tanstack-start';

/**
 * アプリケーションの通常実行時に、サーバー側で使用する Better Auth の認証インスタンス。
 *
 * 認証 API ルートでは auth.handler でリクエストを処理し、
 * サーバー関数では auth.api でセッション取得などの認証処理を行う。
 * DB_AUTH_OPERATOR_NAME・DB_AUTH_OPERATOR_PASSWORD で接続し、auth スキーマ内の認証データを読み書きする。
 * テーブルの作成や構造変更には migration.server.ts のインスタンスを使用する。
 *
 * tanstackStartCookies により、サーバー関数内の認証処理が設定する Cookie をレスポンスに反映する。
 *
 * @see https://better-auth.com/docs/integrations/tanstack
 */
export const auth = betterAuth({
  database: new Pool({
    host: process.env?.DB_HOST,
    port: parseInt(process.env?.DB_PORT ?? '5432', 10),
    database: process.env?.DB_NAME,
    user: process.env?.DB_AUTH_OPERATOR_NAME,
    password: process.env?.DB_AUTH_OPERATOR_PASSWORD,
    options: "-c search_path=auth",
  }),
  emailAndPassword: {
    enabled: true,
  },
  plugins: [
    tanstackStartCookies(),
  ],
});

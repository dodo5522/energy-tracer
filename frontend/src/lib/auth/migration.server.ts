import { betterAuth } from 'better-auth';
import { Pool } from 'pg';

/**
 * Better Auth CLI が認証テーブルのマイグレーションに使用する認証インスタンス。
 *
 * auth migrate の --config にこのファイルを指定して読み込む。
 * DB_MIGRATOR_NAME・DB_MIGRATOR_PASSWORD で接続し、auth スキーマ内のテーブルを作成・更新する。
 * 通常の認証リクエストには operation.server.ts のインスタンスを使用する。
 * 認証方式やプラグインなど、テーブル構造に影響する設定は通常実行用と整合させる。
 *
 * @see https://better-auth.com/docs/concepts/cli
 */
export const auth = betterAuth({
  database: new Pool({
    host: process.env?.DB_HOST,
    port: parseInt(process.env?.DB_PORT ?? '5432', 10),
    database: process.env?.DB_NAME,
    user: process.env?.DB_MIGRATOR_NAME,
    password: process.env?.DB_MIGRATOR_PASSWORD,
    options: "-c search_path=auth",
  }),
  emailAndPassword: {
    enabled: true,
  },
});

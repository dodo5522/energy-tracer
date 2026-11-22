import { createAuthClient } from 'better-auth/react';

/**
 * React コンポーネントから認証 API を利用するための Better Auth クライアント。
 *
 * signUp・signIn・signOut で認証操作を行い、useSession でログイン状態を画面に反映する。
 * 接続先を省略しているため、ブラウザから同一オリジンの /api/auth 配下にリクエストを送信する。
 * 認証処理は、リクエストを受けたサーバー側の auth.handler が実行する。
 *
 * @see https://better-auth.com/docs/concepts/client
 */
export const authClient = createAuthClient();

import { createFileRoute } from '@tanstack/react-router';
import { auth } from '#/lib/auth/operation.server';

/**
 * /api/auth 配下の GET・POST リクエストを受け付け、Better Auth に処理を委譲する。
 *
 * authClient は接続先を省略しているため、ブラウザから同一オリジンのこのルートにアクセスする。
 * 例えば signIn.email() は POST /api/auth/sign-in/email を送信し、
 * auth.handler がパスに応じてログイン処理を実行する。
 *
 * @see https://better-auth.com/docs/concepts/client
 */
export const Route = createFileRoute('/api/auth/$')({
  server: {
    handlers: {
      GET: ({ request }) => auth.handler(request),
      POST: ({ request }) => auth.handler(request),
    },
  },
});

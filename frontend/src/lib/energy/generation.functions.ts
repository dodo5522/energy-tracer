import type { Label } from './types';
import { createServerFn } from '@tanstack/react-start'
import { authMiddleware } from '../auth/middlewares';

/**
 * 認証済みのリクエストに対し、Rust API から登録済みの発電ラベル一覧を取得する。
 *
 * authMiddleware でセッションを検証した後、サーバー側から Rust API を呼び出す。
 * 現在はユーザーによる絞り込みを行わず、全ラベルを取得する。
 *
 * @returns 登録済みの発電ラベル一覧。
 * @throws 未ログインまたはセッション失効時、Rust API が正常なレスポンスを返さない場合に Error。
 */
export const getLabels = createServerFn({ method: 'GET' })
  .middleware([
    authMiddleware,
  ])
  .handler(
    async ({ context }) => {
      const session = context.session;
      console.debug(`${session.user.email}'s session.`);

      const res = await fetch('http://backend:8000/generation/labels', {
        method: 'GET',
      });
      if (!res.ok) {
        throw new Error('Failed to fetch labels');
      }
      return (await res.json()) as Label[];
    }
  );

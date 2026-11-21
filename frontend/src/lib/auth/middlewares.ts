import { createMiddleware } from '@tanstack/react-start';
import { getRequestHeaders } from '@tanstack/react-start/server';
import { auth } from './operation.server';

/**
 * ログイン必須のサーバー関数に適用する認証 middleware。
 *
 * リクエストの Cookie をサーバー側で検証し、有効なセッションがある場合だけ後続の処理を実行する。
 * 検証済みのセッションとユーザー情報は、後続の context.session に渡す。
 * 対象データへのアクセス権限は、後続の処理で別途確認する。
 *
 * @throws 未ログインまたはセッション失効時に Error('Unauthorized')。
 */
export const authMiddleware = createMiddleware({ type: 'function' })
  .server(
    async ({ next }) => {
      const session = await auth.api.getSession({
        headers: getRequestHeaders(),
      });
      if (!session) {
        throw new Error('Unauthorized');
      }
      return next({
        context: { session },
      });
    }
  );

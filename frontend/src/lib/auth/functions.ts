import { createServerFn } from '@tanstack/react-start';
import { getRequestHeaders } from '@tanstack/react-start/server';
import { auth } from './operation.server'

/**
 * リクエストの Cookie をサーバー側で検証し、有効なセッションを取得する。
 *
 * ルートの beforeLoad などから呼び出し、ログイン状態の確認に使用する。
 * この関数はクライアントからも呼び出せるが、セッションの検証はサーバー側で行う。
 * authMiddleware と異なり、未ログインを理由に例外を投げず、呼び出し元で表示や遷移を判断できる。
 *
 * @returns 有効なセッションとユーザー情報。未ログインまたはセッション失効時は null。
 */
export const getSession = createServerFn({ method: 'GET' }).handler(
  async () => {
    return auth.api.getSession({ headers: getRequestHeaders() });
  }
);

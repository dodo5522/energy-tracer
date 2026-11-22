import { createFileRoute, redirect } from '@tanstack/react-router';
import { Configuration } from '#/components/pages';
import { getSession } from '#/lib/auth/functions';

export const Route = createFileRoute('/config')({
  beforeLoad: async () => {
    const session = await getSession();
    if (!session) {
      throw redirect({
        to: '/login',
        search: {
          redirect: '/config'
        },
      });
    }
  },
  component: () => <Configuration search={{redirect: Route.fullPath}}/>,
});

import {createFileRoute} from '@tanstack/react-router';
import {Configuration} from '#/components/pages';
import {labelsGetterQueryOptions} from '#/integrations/home-energy-manager/generation';

export const Route = createFileRoute('/config')({
  loader: ({context}) => {
    return context.queryClient.ensureQueryData(labelsGetterQueryOptions);
  },
  component: () => <Configuration search={{redirect: Route.fullPath}}/>,
});

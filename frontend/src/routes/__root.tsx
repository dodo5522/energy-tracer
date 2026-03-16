import {TanStackDevtools} from '@tanstack/react-devtools';
import type {QueryClient} from '@tanstack/react-query';
import {
  createRootRouteWithContext,
  HeadContent,
  Scripts,
} from '@tanstack/react-router';
import {TanStackRouterDevtoolsPanel} from '@tanstack/react-router-devtools';
import type {ReactNode} from 'react';
import {
  TanStackQueryDevtools,
  TanStackQueryProvider,
} from '#/integrations/tanstack-query';
import appCss from '#/styles.css?url';
import Header from '../components/molecules/Header.tsx';

interface Context {
  queryClient: QueryClient;
}

interface RootDocumentProps {
  children: ReactNode;
  queryClient: QueryClient;
}

interface RootShellProps {
  children: ReactNode;
}

const RootDocument = ({children, queryClient}: RootDocumentProps) => {
  return (
    <html lang="en">
    <head>
      <HeadContent/>
    </head>
    <body>
    <TanStackQueryProvider queryClient={queryClient}>
      <Header/>
      {children}
      <TanStackDevtools
        config={{
          position: 'bottom-right',
        }}
        plugins={[
          {
            name: 'Tanstack Router',
            render: <TanStackRouterDevtoolsPanel/>,
          },
          TanStackQueryDevtools,
        ]}
      />
    </TanStackQueryProvider>
    <Scripts/>
    </body>
    </html>
  );
};

const RootShell = ({children}: RootShellProps) => {
  const queryClient = Route.useRouteContext({
    select: (context) => context.queryClient,
  });
  return <RootDocument queryClient={queryClient}>{children}</RootDocument>;
};

export const Route = createRootRouteWithContext<Context>()({
  head: () => ({
    meta: [
      {
        charSet: 'utf-8',
      },
      {
        name: 'viewport',
        content: 'width=device-width, initial-scale=1',
      },
      {
        title: 'Home energy manager',
      },
    ],
    links: [
      {
        rel: 'stylesheet',
        href: appCss,
      },
    ],
  }),
  shellComponent: RootShell,
});

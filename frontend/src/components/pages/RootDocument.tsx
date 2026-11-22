import { TanStackDevtools } from '@tanstack/react-devtools';
import { HeadContent, Scripts } from '@tanstack/react-router';
import { TanStackRouterDevtoolsPanel } from '@tanstack/react-router-devtools';
import { type ReactNode, useState } from 'react';
import { AppBar }  from '#/components/molecules';
import { Drawer }  from '#/components/organisms';
import {
  TanStackQueryDevtools,
  TanStackQueryProvider,
} from '#/integrations/tanstack-query';

const RootDocument = ({ children }: { children: ReactNode }) => {
  const [isDrawerOpened, setIsDrawerOpened] = useState(false);

  return (
    <html lang="en">
      <head>
        <HeadContent/>
      </head>
      <body>
        <TanStackQueryProvider>
          <AppBar
            onClickToOpenMenu={() => setIsDrawerOpened(true)}
          />
          <Drawer
            isOpened={isDrawerOpened}
            onClickToClose={() => setIsDrawerOpened(false)}
          />
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

export default RootDocument;

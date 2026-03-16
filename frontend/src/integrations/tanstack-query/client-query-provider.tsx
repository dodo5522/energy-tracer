import {type QueryClient, QueryClientProvider} from '@tanstack/react-query';
import type {ReactNode} from 'react';

interface TanStackQueryProviderProps {
  children: ReactNode;
  queryClient: QueryClient;
}

const TanStackQueryProvider = ({
                                 children,
                                 queryClient,
                               }: TanStackQueryProviderProps) => {
  return (
    <QueryClientProvider client={queryClient}>{children}</QueryClientProvider>
  );
};

export default TanStackQueryProvider;

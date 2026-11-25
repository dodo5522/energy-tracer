import {
  Box,
  List,
  ListItem,
  Paper,
  Stack,
  Typography,
} from '@mui/material';
import {useQuery} from '@tanstack/react-query';
import {Navigate} from '@tanstack/react-router';
import {RefreshCw} from 'lucide-react';
import {AlertSession, IconButton, Loading} from '#/components/atoms';
import {getLabels} from '#/lib/energy/generation.functions';
import {authClient} from '#/lib/auth/client';
import type { LoginSearch } from '#/types';

interface ConfigurationProps {
  search: LoginSearch;
}

const Configuration = ({search}: ConfigurationProps) => {
  const {
    data: session,
    isPending: isPendingSession,
    isRefetching,
    error: errorSession,
    refetch: refetchSession,
  } = authClient.useSession();
  const {
    data: labels = [],
    isFetching,
    isPending: isPendingLabels,
    refetch: refetchLabels,
    error: errorLabels,
  } = useQuery({
    queryKey: ['labels'],
    queryFn: getLabels,
    enabled: !isPendingSession && !!session?.user,
  });
  const isRefreshing = isFetching || isRefetching;

  if (isPendingSession) {
    return <Loading/>;
  } else if (!errorSession) {
    if (!session?.user) {
      return <Navigate to="/login" search={search} replace/>;
    } else if (isPendingLabels) {
      return <Loading/>;
    }
  }

  return (
    <Box
      sx={{
        minHeight: '100vh',
        px: 2,
        py: 4,
        display: 'flex',
        alignItems: 'start',
        justifyContent: 'center',
        color: 'common.white',
        backgroundImage:
          'radial-gradient(50% 50% at 95% 5%, #f4a460 0%, #8b4513 70%, #1a0f0a 100%)',
      }}
    >
      <Paper
        elevation={8}
        sx={{
          width: '100%',
          maxWidth: '90%',
          p: 4,
          borderRadius: 2,
          backdropFilter: 'blur(10px)',
          bgcolor: 'rgba(0, 0, 0, 0.5)',
          border: '8px solid rgba(0, 0, 0, 0.1)',
          color: 'common.white',
        }}
      >
        <Stack
          direction="row"
          alignItems="center"
          justifyContent="flex-start"
          spacing={1}
          mb={2}
        >
          <Typography variant="h5" sx={{mb: 2}}>
            Labels
          </Typography>
          <IconButton
            size="large"
            edge="start"
            color="inherit"
            isSpinning={isRefreshing}
            disabled={isRefreshing}
            aria-label={isRefreshing ? '更新中' : '再取得'}
            aria-busy={isRefreshing}
            onClick={async () => {
              if (errorSession) {
                await refetchSession();
              } else {
                void refetchLabels();
              }
            }}
            // refer to: https://mui.com/material-ui/customization/how-to-customize/#state-classes
            sx={{
              '&.Mui-disabled': {
                color: 'inherit',
              },
            }}
          >
            <RefreshCw size={14}/>
          </IconButton>
        </Stack>

        <AlertSession hasSessionError={!!errorSession} anotherError={errorLabels?.message} />

        {!errorSession && !errorLabels
          ? <List sx={{ p: 0 }}>
            {labels.map((label) => (
              <ListItem
                key={label.label}
                sx={{
                  mb: 1,
                  borderRadius: 1.5,
                  px: 2,
                  py: 1.25,
                  border: '1px solid rgba(255, 255, 255, 0.2)',
                  bgcolor: 'rgba(255, 255, 255, 0.1)',
                  backdropFilter: 'blur(8px)',
                  boxShadow: 1,
                }}
              >
                <Stack direction="row" spacing={2}>
                  <Typography variant="body1" sx={{ color: 'common.white' }}>
                    {label.label}
                  </Typography>
                  <Typography variant="body1" sx={{ color: 'common.white' }}>
                    {label.remark}
                  </Typography>
                </Stack>
              </ListItem>
            ))}
          </List>
          : null}
      </Paper>
    </Box>
  );
};

export default Configuration;

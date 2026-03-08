import {
  Avatar,
  Box,
  Button,
  Card,
  CardContent,
  Stack,
  Typography,
} from '@mui/material';
import {Navigate} from '@tanstack/react-router';
import {Loading} from '#/components/atoms';
import {TemplateBody} from '#/components/templates';
import {authClient} from '#/lib/auth-client';
import type {LoginSearch} from '#/types';

interface TopProps {
  search: LoginSearch;
}

const Top = ({search}: TopProps) => {
  const {data: session, isPending} = authClient.useSession();

  if (isPending) {
    return <Loading/>;
  }
  if (!session?.user) {
    return <Navigate to="/login" search={search} replace/>;
  }

  return (
    <TemplateBody>
      <Card
        sx={{
          width: '100%',
          maxWidth: 420,
          border: '8px solid rgba(0, 0, 0, 0.1)',
        }}
      >
        <CardContent>
          <Stack spacing={3}>
            <Box>
              <Typography variant="h6" fontWeight={600}>
                Welcome back
              </Typography>
              <Typography variant="body2" color="text.secondary">
                You're signed in as {session.user.email}
              </Typography>
            </Box>

            <Stack direction="row" spacing={1.5} alignItems="center">
              {session.user.image ? (
                <Avatar
                  src={session.user.image}
                  alt={session.user.name || 'User'}
                  sx={{width: 40, height: 40}}
                />
              ) : (
                <Avatar sx={{width: 40, height: 40}}>
                  {(session.user.name?.charAt(0) || 'U').toUpperCase()}
                </Avatar>
              )}
              <Box sx={{minWidth: 0}}>
                <Typography variant="body2" fontWeight={600} noWrap>
                  {session.user.name}
                </Typography>
                <Typography variant="caption" color="text.secondary" noWrap>
                  {session.user.email}
                </Typography>
              </Box>
            </Stack>

            <Button
              variant="outlined"
              onClick={() => {
                void authClient.signOut();
              }}
            >
              Sign out
            </Button>
          </Stack>
        </CardContent>
      </Card>
    </TemplateBody>
  );
};

export default Top;

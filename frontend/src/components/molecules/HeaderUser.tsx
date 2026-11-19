import {
  Avatar,
  Box,
  Button,
  Skeleton,
  Stack,
  Typography,
} from '@mui/material';
import {Link, useNavigate} from '@tanstack/react-router';
import {authClient} from '#/lib/auth/client';

const BetterAuthHeader = () => {
  const navigate = useNavigate();
  const {data: session, isPending} = authClient.useSession();

  if (isPending) {
    return (
      <Skeleton
        variant="rounded"
        width={120}
        height={36}
        sx={{bgcolor: 'grey.700'}}
      />
    );
  } else if (session?.user) {
    return (
      <Stack direction="row" spacing={1} alignItems="center">
        {session.user.image ? (
          <Avatar
            src={session.user.image}
            alt={session.user.name || 'User'}
            sx={{width: 32, height: 32}}
          />
        ) : (
          <Avatar sx={{width: 32, height: 32}}>
            <Typography variant="caption" fontWeight={600}>
              {session.user.name?.charAt(0).toUpperCase() || 'U'}
            </Typography>
          </Avatar>
        )}
        <Button
          variant="outlined"
          size="small"
          onClick={() => {
            void authClient.signOut().then(() => {
              void navigate({to: '/login'});
            });
          }}
          sx={{
            minHeight: 36,
            color: 'common.white',
            borderColor: 'grey.600',
            '&:hover': {borderColor: 'grey.500', bgcolor: 'grey.800'},
          }}
        >
          Sign out
        </Button>
      </Stack>
    );
  } else {
    return (
      <Box>
        <Link to="/login" search={{redirect: '/'}}>
          <Button
            variant="outlined"
            size="small"
            sx={{
              minHeight: 36,
              color: 'common.white',
              borderColor: 'grey.600',
              '&:hover': {borderColor: 'grey.500', bgcolor: 'grey.800'},
            }}
          >
            Sign in
          </Button>
        </Link>
      </Box>
    );
  }
};

export default BetterAuthHeader;

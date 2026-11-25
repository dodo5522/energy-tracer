import {Box} from '@mui/material';
import {Navigate} from '@tanstack/react-router';
import {useState} from 'react';
import {Loading} from '#/components/atoms';
import {CardSignIn, CardSignUp} from '#/components/molecules';
import {authClient} from '#/lib/auth/client';

interface LogInProps {
  redirectTo: string;
}

const LogIn = ({redirectTo}: LogInProps) => {
  const {
    data: session,
    error,
    isPending,
    refetch,
  } = authClient.useSession();
  const [isSignUp, setIsSignUp] = useState(false);

  if (isPending) {
    return <Loading/>;
  } else if (session?.user) {
    return <Navigate to={redirectTo} replace/>;
  }

  return (
    <Box sx={{display: 'flex', justifyContent: 'center', py: 10, px: 2}}>
      {isSignUp
        ? <CardSignUp
          onClickIntoSignIn={() => setIsSignUp(false)}
          onRetrySession={() => refetch()}
          hasSessionError={ !!error }
        />
        : <CardSignIn
          onClickIntoSignUp={() => setIsSignUp(true)}
          onRetrySession={() => refetch()}
          hasSessionError={ !!error }
        />
      }
    </Box>
  );
};

export default LogIn;

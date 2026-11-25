import {
  Box,
  Button,
  Card,
  CardContent,
  CircularProgress,
  Stack,
  TextField,
  Typography,
} from '@mui/material';
import {type SubmitEvent, useState} from 'react';
import {authClient} from '#/lib/auth/client';
import { AlertSession } from '../atoms';

interface CardSignInProps {
  onClickIntoSignUp: () => void;
  onRetrySession: () => Promise<void>;
  hasSessionError: boolean;
}

interface CardSignInState {
  error?: string
  loading?: boolean;
}

const CardSignIn = (props: CardSignInProps) => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [state, setState] = useState<CardSignInState>({});

  const handleSubmit = async (e: SubmitEvent) => {
    e.preventDefault();
    setState({ loading: true });

    try {
      if (props.hasSessionError) {
        await props.onRetrySession();
        return;
      }

      const result = await authClient.signIn.email({
        email,
        password,
      });
      if (result.error) {
        setState({ error: result.error.message || 'Sign in failed' });
        return;
      }

      setState({});
    } catch (_) {
      setState({ error: 'An unexpected error occurred' });
    } finally {
      setState((prev) => ({...prev, loading: false}));
    }
  };

  return (
    <Card sx={{width: '100%', maxWidth: 420}}>
      <CardContent>
        <Stack spacing={2.5}>
          <Box>
            <Typography variant="h6" fontWeight={600}>
              Sign in
            </Typography>
            <Typography variant="body2" color="text.secondary" sx={{mt: 1}}>
              Sign in to open the protected page
            </Typography>
          </Box>

          <Box component="form" onSubmit={handleSubmit}>
            <Stack spacing={2}>
              <TextField
                label="Email"
                type="email"
                autoComplete="email"
                disabled={props.hasSessionError}
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                required
                fullWidth
                size="small"
              />
              <TextField
                label="Password"
                type="password"
                autoComplete={'current-password'}
                disabled={props.hasSessionError}
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                required
                fullWidth
                size="small"
                slotProps={{htmlInput: {minLength: 8}}}
              />

              <AlertSession hasSessionError={props.hasSessionError} anotherError={state.error} />

              <Button
                type="submit"
                disabled={!!state.loading}
                variant="contained"
                fullWidth
              >
                {
                  state.loading ? (
                    <Stack direction="row" spacing={1} alignItems="center">
                      <CircularProgress size={16} color="inherit"/>
                      <span>Please wait</span>
                    </Stack>
                  ) : (props.hasSessionError ? 'Retry session' : 'Sign in')
                }
              </Button>
            </Stack>
          </Box>

          <Button
            type="button"
            variant="text"
            disabled={!!state.loading}
            onClick={() => {
              props.onClickIntoSignUp();
              setState({});
            }}
          >
            Don't have an account? Sign up
          </Button>
        </Stack>
      </CardContent>
    </Card>
  );
};

export default CardSignIn;

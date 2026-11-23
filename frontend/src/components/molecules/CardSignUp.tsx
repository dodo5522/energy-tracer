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
import { authClient } from '#/lib/auth/client';
import { AlertSession } from '../atoms';

interface CardSignUpProps {
  onClickIntoSignIn: () => void;
  onRetrySession: () => Promise<void>;
  hasSessionError: boolean;
}

interface CardSignUpState {
  error?: string
  loading?: boolean;
}

const CardSignUp = (props: CardSignUpProps ) => {
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [name, setName] = useState('');
  const [state, setState] = useState<CardSignUpState>({});

  const handleSubmit = async (e: SubmitEvent) => {
    e.preventDefault();
    setState({loading: true});

    try {
      if (props.hasSessionError) {
        await props.onRetrySession();
        return;
      }

      const result = await authClient.signUp.email({
        email,
        password,
        name,
      });
      if (result.error) {
        setState({ error: result.error.message || 'Sign up failed' });
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
              Create an account
            </Typography>
            <Typography variant="body2" color="text.secondary" sx={{mt: 1}}>
              Enter your information to create an account
            </Typography>
          </Box>

          <Box component="form" onSubmit={handleSubmit}>
            <Stack spacing={2}>
              <TextField
                label="Name"
                autoComplete="username"
                disabled={props.hasSessionError}
                value={name}
                onChange={(e) => setName(e.target.value)}
                required
                fullWidth
                size="small"
              />
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
                autoComplete="new-password"
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
                  ) : (props.hasSessionError ? 'Retry session' : 'Create account')
                }
              </Button>
            </Stack>
          </Box>

          <Button
            type="button"
            variant="text"
            disabled={!!state.loading}
            onClick={() => {
              props.onClickIntoSignIn();
              setState({});
            }}
          >
            Already have an account? Sign in
          </Button>
        </Stack>
      </CardContent>
    </Card>
  );
};

export default CardSignUp;

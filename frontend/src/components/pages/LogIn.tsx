import {
  Alert,
  Box,
  Button,
  Card,
  CardContent,
  CircularProgress,
  Stack,
  TextField,
  Typography,
} from '@mui/material';
import {Navigate} from '@tanstack/react-router';
import {type SubmitEvent, useState} from 'react';
import {Loading} from '#/components/atoms';
import {TemplateBody} from '#/components/templates';
import {authClient} from '#/lib/auth-client';

interface LogInProps {
  redirectTo: string;
}

const LogIn = ({redirectTo}: LogInProps) => {
  const {data: session, isPending} = authClient.useSession();
  const [isSignUp, setIsSignUp] = useState(false);
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [name, setName] = useState('');
  const [error, setError] = useState('');
  const [loading, setLoading] = useState(false);

  if (isPending) {
    return <Loading/>;
  }
  if (session?.user) {
    return <Navigate to={redirectTo} replace/>;
  }

  const handleSubmit = async (e: SubmitEvent) => {
    e.preventDefault();
    setError('');
    setLoading(true);

    try {
      if (isSignUp) {
        const result = await authClient.signUp.email({
          email,
          password,
          name,
        });
        if (result.error) {
          setError(result.error.message || 'Sign up failed');
          return;
        }
      } else {
        const result = await authClient.signIn.email({
          email,
          password,
        });
        if (result.error) {
          setError(result.error.message || 'Sign in failed');
          return;
        }
      }
    } catch (_) {
      setError('An unexpected error occurred');
    } finally {
      setLoading(false);
    }
  };

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
          <Stack spacing={2.5}>
            <Box>
              <Typography variant="h6" fontWeight={600}>
                {isSignUp ? 'Create an account' : 'Sign in'}
              </Typography>
              <Typography variant="body2" color="text.secondary" sx={{mt: 1}}>
                {isSignUp
                  ? 'Enter your information to create an account'
                  : 'Sign in to open the protected page'}
              </Typography>
            </Box>

            <Box component="form" onSubmit={handleSubmit}>
              <Stack spacing={2}>
                {isSignUp && (
                  <TextField
                    label="Name"
                    autoComplete="username"
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    required
                    fullWidth
                    size="small"
                  />
                )}
                <TextField
                  label="Email"
                  type="email"
                  autoComplete="email"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  required
                  fullWidth
                  size="small"
                />
                <TextField
                  label="Password"
                  type="password"
                  autoComplete={isSignUp ? 'new-password' : 'current-password'}
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  required
                  fullWidth
                  size="small"
                  slotProps={{htmlInput: {minLength: 8}}}
                />

                {error && <Alert severity="error">{error}</Alert>}

                <Button
                  type="submit"
                  disabled={loading}
                  variant="contained"
                  fullWidth
                >
                  {loading ? (
                    <Stack direction="row" spacing={1} alignItems="center">
                      <CircularProgress size={16} color="inherit"/>
                      <span>Please wait</span>
                    </Stack>
                  ) : isSignUp ? (
                    'Create account'
                  ) : (
                    'Sign in'
                  )}
                </Button>
              </Stack>
            </Box>

            <Button
              type="button"
              variant="text"
              onClick={() => {
                setIsSignUp(!isSignUp);
                setError('');
              }}
            >
              {isSignUp
                ? 'Already have an account? Sign in'
                : "Don't have an account? Sign up"}
            </Button>
          </Stack>
        </CardContent>
      </Card>
    </TemplateBody>
  );
};

export default LogIn;

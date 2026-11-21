import {
  AppBar as MuiAppBar,
  Box,
  IconButton,
  Skeleton,
  Toolbar,
} from '@mui/material';
import { Link } from '@tanstack/react-router';
import { Menu } from 'lucide-react';
import { authClient } from '#/lib/auth/client';
import { Offset } from '#/components/atoms';

interface AppBarProps {
  onClickToOpenMenu: () => void,
}

const AppBar = (props: AppBarProps) => {
  const {
    data: session,
    isPending,
  } = authClient.useSession();

  if (!session?.user || isPending) {
    return (
      <>
        <MuiAppBar position="sticky" elevation={4} sx={{bgcolor: 'grey.900'}}>
          <Toolbar sx={{minHeight: 72}}>
            <Skeleton
              variant="rounded"
              width={40}
              height={40}
              sx={{bgcolor: 'grey.900'}}
            />
            <Box sx={{ml: 2, display: 'inline-flex', alignItems: 'center'}}>
              <Box
                component="img"
                src="/tanstack-word-logo-white.svg"
                alt="TanStack Logo"
                sx={{height: 40}}
              />
            </Box>
          </Toolbar>
        </MuiAppBar>
        <Offset/>
      </>
    );
  }

  return (
    <>
      <MuiAppBar position="fixed" elevation={4} sx={{bgcolor: 'grey.900'}}>
        <Toolbar sx={{minHeight: 72}}>
          <IconButton
            size="large"
            edge="start"
            color="inherit"
            onClick={() => props.onClickToOpenMenu()}
            aria-label="Open menu"
            sx={{mr: 1}}
          >
            <Menu size={24}/>
          </IconButton>
          <Box sx={{display: 'inline-flex', alignItems: 'center'}}>
            <Link to="/">
              <Box
                component="img"
                src="/tanstack-word-logo-white.svg"
                alt="TanStack Logo"
                sx={{height: 40}}
              />
            </Link>
          </Box>
        </Toolbar>
      </MuiAppBar>
      <Offset/>
    </>
  );
};

export default AppBar;

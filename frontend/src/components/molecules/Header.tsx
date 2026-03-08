import {
  AppBar,
  Box,
  Divider,
  Drawer,
  IconButton,
  List,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Skeleton,
  styled,
  Toolbar,
  Typography,
} from '@mui/material';
import {Link, useNavigate} from '@tanstack/react-router';
import {ChartLine, Menu, Settings, X} from 'lucide-react';
import {useState} from 'react';

import {authClient} from '#/lib/auth-client.ts';
import BetterAuthHeader from './HeaderUser.tsx';

const Offset = styled('div')(({theme}) => theme.mixins.toolbar);

const Header = () => {
  const navigate = useNavigate();
  const [isOpen, setIsOpen] = useState(false);
  const {data: session, isPending} = authClient.useSession();

  if (!session?.user || isPending) {
    return (
      <>
        <AppBar position="fixed" elevation={4} sx={{bgcolor: 'grey.900'}}>
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
        </AppBar>
        <Offset/>
      </>
    );
  }

  const handleNavigate = (to: string) => {
    setIsOpen(false);
    void navigate({to});
  };

  return (
    <>
      <AppBar position="fixed" elevation={4} sx={{bgcolor: 'grey.900'}}>
        <Toolbar sx={{minHeight: 72}}>
          <IconButton
            size="large"
            edge="start"
            color="inherit"
            onClick={() => setIsOpen(true)}
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
      </AppBar>
      <Offset/>

      <Drawer
        anchor="left"
        open={isOpen}
        onClose={() => setIsOpen(false)}
        slotProps={{
          paper: {
            sx: {
              width: 250,
              bgcolor: 'grey.900',
              color: 'common.white',
              display: 'flex',
              flexDirection: 'column',
            },
          },
        }}
      >
        <Box
          sx={{
            px: 2,
            py: 1.5,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'space-between',
            borderBottom: 1,
            borderColor: 'grey.700',
          }}
        >
          <Typography variant="h6" component="h2" sx={{fontWeight: 700}}>
            Menu
          </Typography>
          <IconButton
            size="large"
            onClick={() => setIsOpen(false)}
            aria-label="Close menu"
            sx={{color: 'inherit'}}
          >
            <X size={24}/>
          </IconButton>
        </Box>

        <List sx={{px: 1, py: 1}}>
          <ListItemButton
            onClick={() => handleNavigate('/')}
            sx={{borderRadius: 1, mb: 0.5}}
          >
            <ListItemIcon sx={{color: 'inherit', minWidth: 36}}>
              <ChartLine size={20}/>
            </ListItemIcon>
            <ListItemText primary="Home"/>
          </ListItemButton>

          <ListItemButton
            onClick={() => handleNavigate('/config')}
            sx={{borderRadius: 1, mb: 0.5}}
          >
            <ListItemIcon sx={{color: 'inherit', minWidth: 36}}>
              <Settings size={20}/>
            </ListItemIcon>
            <ListItemText primary="Configuration"/>
          </ListItemButton>
        </List>

        <Divider sx={{borderColor: 'grey.700', mt: 'auto'}}/>
        <Box sx={{p: 2}}>
          <BetterAuthHeader/>
        </Box>
      </Drawer>
    </>
  );
};

export default Header;

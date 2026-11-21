import {
  Box,
  Divider,
  Drawer as MuiDrawer,
  IconButton,
  List,
  ListItemButton,
  ListItemIcon,
  ListItemText,
  Typography,
} from '@mui/material';
import {useNavigate} from '@tanstack/react-router';
import {ChartLine, Settings, X} from 'lucide-react';
import {authClient} from '#/lib/auth/client';
import HeaderUser from './HeaderUser';

interface DrawerProps {
  isOpened: boolean,
  onClickToClose: () => void,
}

const Drawer = (props: DrawerProps) => {
  const navigate = useNavigate();
  const {data: session, isPending} = authClient.useSession();

  if (!session?.user || isPending) {
    return (<></>);
  }

  const handleNavigate = (to: string) => {
    props.onClickToClose();
    void navigate({to});
  };

  return (
    <MuiDrawer
      anchor="left"
      open={props.isOpened}
      onClose={() => props.onClickToClose()}
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
          onClick={() => props.onClickToClose()}
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
        <HeaderUser/>
      </Box>
    </MuiDrawer>
  );
};

export default Drawer;

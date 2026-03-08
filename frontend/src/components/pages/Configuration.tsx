import {
  IconButton,
  List,
  ListItem,
  Paper,
  Stack,
  TextField,
  Typography,
} from '@mui/material';
import {useQuery} from '@tanstack/react-query';
import {Navigate} from '@tanstack/react-router';
import {RefreshCw} from 'lucide-react';
import {Loading} from '#/components/atoms';
import {TemplateBody} from '#/components/templates';
import {getLabels} from '#/integrations/home-energy-manager/generation';
import {authClient} from '#/lib/auth-client.ts';
import type {LoginSearch} from '#/types';

interface ConfigurationProps {
  search: LoginSearch;
}

const Configuration = ({search}: ConfigurationProps) => {
  const {data: session} = authClient.useSession();
  const {
    data: labels = [],
    isFetching,
    isPending,
    refetch,
  } = useQuery({
    queryKey: ['labels'],
    queryFn: getLabels,
  });

  if (isPending) {
    return <Loading/>;
  }
  if (!session?.user) {
    return <Navigate to="/login" search={search} replace/>;
  }

  return (
    <TemplateBody>
      <Paper
        elevation={8}
        sx={{
          width: '100%',
          maxWidth: 768,
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
            disabled={isFetching}
            onClick={() => void refetch()}
          >
            <RefreshCw size={14}/>
          </IconButton>
        </Stack>
        <List sx={{p: 0}}>
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
                color: 'common.white',
              }}
            >
              <Stack direction="row" spacing={2}>
                <TextField
                  placeholder="label"
                  size="small"
                  defaultValue={label.label}
                  slotProps={{
                    input: {
                      readOnly: true,
                      style: {color: 'white'},
                    },
                  }}
                />
                <TextField
                  placeholder="remark"
                  size="small"
                  defaultValue={label.remark}
                  slotProps={{
                    input: {
                      readOnly: true,
                      style: {color: 'white'},
                    },
                  }}
                />
              </Stack>
            </ListItem>
          ))}
        </List>
      </Paper>
    </TemplateBody>
  );
};

export default Configuration;

import {Box} from '@mui/material';
import type {ReactNode} from 'react';

const TemplateBody = ({children}: { children: ReactNode }) => {
  return (
    <Box
      sx={{
        minHeight: '100vh',
        px: 2,
        py: 4,
        display: 'flex',
        alignItems: 'start',
        justifyContent: 'center',
        color: 'common.white',
        backgroundImage:
          'radial-gradient(50% 50% at 95% 5%, #f4a460 0%, #8b4513 70%, #1a0f0a 100%)',
      }}
    >
      {children}
    </Box>
  );
};

export default TemplateBody;

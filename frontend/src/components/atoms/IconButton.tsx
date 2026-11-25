import { IconButton as MuiIconButton, type IconButtonProps as MuiIconButtonProps } from '@mui/material';

interface IconButtonProps extends MuiIconButtonProps {
  isSpinning: boolean;
}

// refer to: https://mui.com/system/getting-started/the-sx-prop/#passing-the-sx-prop
const IconButton = ({isSpinning, sx = [], ...props}: IconButtonProps) => {
  return <MuiIconButton
    {...props}
    sx={[
      {
        '@keyframes refresh-spin': {
          to: { transform: 'rotate(360deg)' },
        },
        '& svg': {
          animation: isSpinning
            ? 'refresh-spin 1s linear infinite'
            : 'none',
        },
        '@media (prefers-reduced-motion: reduce)': {
          '& svg': { animation: 'none' },
        },
      },
      ...[sx].flat(),
    ]}
  >
    {props.children}
  </MuiIconButton>
};

export default IconButton;

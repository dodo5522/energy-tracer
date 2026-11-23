import { Alert, Stack } from '@mui/material';

interface AlertSessionProps {
  hasSessionError: boolean;
  anotherError?: string;
}

const AlertSession = (props: AlertSessionProps) => {
  if (!props.hasSessionError && !props.anotherError) {
    return null;
  }

  return <Stack spacing={2}>
    {props.hasSessionError && <Alert severity="error">ログイン状態を確認できませんでした。<br/>時間をおいて再試行してください。</Alert>}
    {props.anotherError && <Alert severity="error">{props.anotherError}</Alert>}
  </Stack>;
};

export default AlertSession;

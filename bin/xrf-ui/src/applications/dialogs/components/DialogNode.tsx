import { Paper, Stack, TextField, Typography } from "@mui/material";
import { ChangeEvent, ReactElement, useCallback } from "react";
import { Handle, Position } from "reactflow";

import { AnyObject } from "@/core/types/general";

interface IDialogNodeProps {
  data: AnyObject;
  isConnectable?: boolean;
}

export function DialogNode({ data, isConnectable }: IDialogNodeProps): ReactElement {
  const onChange = useCallback((event: ChangeEvent<HTMLInputElement>) => {
    console.log(event.target.value);
  }, []);

  return (
    <Paper variant={"outlined"} sx={{ padding: 1.5, minWidth: 200 }}>
      <Typography variant={"subtitle2"} gutterBottom>
        {data.label}
      </Typography>

      <Stack spacing={1}>
        <TextField className={"nodrag"} label={"Has info"} size={"small"} fullWidth onChange={onChange} />
        <TextField className={"nodrag"} label={"No info"} size={"small"} fullWidth onChange={onChange} />
      </Stack>

      <Handle type={"source"} position={Position.Bottom} isConnectable={isConnectable} />
    </Paper>
  );
}

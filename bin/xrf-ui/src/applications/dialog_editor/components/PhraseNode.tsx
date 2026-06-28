import { Paper, Stack, TextField, Typography } from "@mui/material";
import { ChangeEvent, ReactElement, useCallback } from "react";
import { Handle, Position } from "reactflow";

import { AnyObject } from "@/core/types/general";

interface IPhraseNodeProps {
  data: AnyObject;
  isConnectable?: boolean;
}

export function PhraseNode({ data, isConnectable }: IPhraseNodeProps): ReactElement {
  const onChange = useCallback((event: ChangeEvent<HTMLInputElement>) => {
    console.log(event.target.value);
  }, []);

  return (
    <Paper variant={"outlined"} sx={{ padding: 1.5, minWidth: 200 }}>
      <Handle type={"source"} position={Position.Top} isConnectable={isConnectable} />

      <Typography variant={"subtitle2"} gutterBottom>
        {data.label}
      </Typography>

      <Stack spacing={1}>
        {["Text", "Action", "Precondition", "Give info", "Disable info", "Is final"].map((field) => (
          <TextField key={field} className={"nodrag"} label={field} size={"small"} fullWidth onChange={onChange} />
        ))}
      </Stack>

      <Handle type={"source"} position={Position.Bottom} isConnectable={isConnectable} />
    </Paper>
  );
}

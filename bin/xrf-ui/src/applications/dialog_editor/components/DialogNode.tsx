import { Grid } from "@mui/material";
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
    <Grid bgcolor={"red"} padding={2} component={"div"}>
      <Grid>{data.label}</Grid>

      <br />

      <Grid direction={"column"} container>
        <label htmlFor={"text"}>Has info:</label>
        <input className={"nodrag"} onChange={onChange} />
      </Grid>

      <Grid direction={"column"} container>
        <label htmlFor={"text"}>No info:</label>
        <input className={"nodrag"} onChange={onChange} />
      </Grid>

      <Handle type={"source"} position={Position.Bottom} isConnectable={isConnectable} />
    </Grid>
  );
}

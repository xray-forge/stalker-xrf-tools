import { Grid } from "@mui/material";
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
    <Grid bgcolor={"red"} padding={2} component={"div"}>
      <Grid>{data.label}</Grid>

      <Handle type={"source"} position={Position.Top} isConnectable={isConnectable} />

      <br />

      <Grid direction={"column"} container>
        <label htmlFor={"text"}>Text:</label>
        <input className={"nodrag"} onChange={onChange} />
      </Grid>

      <Grid direction={"column"} container>
        <label htmlFor={"text"}>Action:</label>
        <input className={"nodrag"} onChange={onChange} />
      </Grid>

      <Grid direction={"column"} container>
        <label htmlFor={"text"}>Precondition:</label>
        <input className={"nodrag"} onChange={onChange} />
      </Grid>

      <Grid direction={"column"} container>
        <label htmlFor={"text"}>Give info:</label>
        <input className={"nodrag"} onChange={onChange} />
      </Grid>

      <Grid direction={"column"} container>
        <label htmlFor={"text"}>Disable info:</label>
        <input className={"nodrag"} onChange={onChange} />
      </Grid>

      <Grid direction={"column"} container>
        <label htmlFor={"text"}>Is final:</label>
        <input className={"nodrag"} onChange={onChange} />
      </Grid>

      <Handle type={"source"} position={Position.Bottom} isConnectable={isConnectable} />
    </Grid>
  );
}

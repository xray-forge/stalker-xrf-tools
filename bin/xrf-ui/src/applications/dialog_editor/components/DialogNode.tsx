import { Box } from "@mui/material";
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
    <Box component={"div"} sx={{ bgcolor: "red", padding: 2 }}>
      <Box>{data.label}</Box>

      <br />

      <Box sx={{ display: "flex", flexDirection: "column" }}>
        <label htmlFor={"text"}>Has info:</label>
        <input className={"nodrag"} onChange={onChange} />
      </Box>

      <Box sx={{ display: "flex", flexDirection: "column" }}>
        <label htmlFor={"text"}>No info:</label>
        <input className={"nodrag"} onChange={onChange} />
      </Box>

      <Handle type={"source"} position={Position.Bottom} isConnectable={isConnectable} />
    </Box>
  );
}

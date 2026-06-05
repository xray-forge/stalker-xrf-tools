import { Box } from "@mui/material";
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
    <Box component={"div"} sx={{ bgcolor: "red", padding: 2 }}>
      <Box>{data.label}</Box>

      <Handle type={"source"} position={Position.Top} isConnectable={isConnectable} />

      <br />

      <Box sx={{ display: "flex", flexDirection: "column" }}>
        <label htmlFor={"text"}>Text:</label>
        <input className={"nodrag"} onChange={onChange} />
      </Box>

      <Box sx={{ display: "flex", flexDirection: "column" }}>
        <label htmlFor={"text"}>Action:</label>
        <input className={"nodrag"} onChange={onChange} />
      </Box>

      <Box sx={{ display: "flex", flexDirection: "column" }}>
        <label htmlFor={"text"}>Precondition:</label>
        <input className={"nodrag"} onChange={onChange} />
      </Box>

      <Box sx={{ display: "flex", flexDirection: "column" }}>
        <label htmlFor={"text"}>Give info:</label>
        <input className={"nodrag"} onChange={onChange} />
      </Box>

      <Box sx={{ display: "flex", flexDirection: "column" }}>
        <label htmlFor={"text"}>Disable info:</label>
        <input className={"nodrag"} onChange={onChange} />
      </Box>

      <Box sx={{ display: "flex", flexDirection: "column" }}>
        <label htmlFor={"text"}>Is final:</label>
        <input className={"nodrag"} onChange={onChange} />
      </Box>

      <Handle type={"source"} position={Position.Bottom} isConnectable={isConnectable} />
    </Box>
  );
}

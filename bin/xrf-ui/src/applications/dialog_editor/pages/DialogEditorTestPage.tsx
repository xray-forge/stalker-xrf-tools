import { Grid } from "@mui/material";
import { ReactElement, useCallback } from "react";
import { addEdge, Background, Connection, Controls, Edge, ReactFlow, useEdgesState, useNodesState } from "reactflow";

import { DialogNode } from "@/applications/dialog_editor/components/DialogNode";
import { PhraseNode } from "@/applications/dialog_editor/components/PhraseNode";
import { EGraphNodeType } from "@/applications/dialog_editor/types";

import "reactflow/dist/style.css";

const initialNodes = [
  { id: "dialog", type: EGraphNodeType.DIALOG_NODE, position: { x: 10, y: 15 }, data: { label: "dialog node" } },
  { id: "0", type: EGraphNodeType.PHRASE_NODE, position: { x: 10, y: 250 }, data: { label: "phrase 0" } },
  { id: "1", type: EGraphNodeType.PHRASE_NODE, position: { x: 230, y: 250 }, data: { label: "phrase 1" } },
  { id: "01", type: EGraphNodeType.PHRASE_NODE, position: { x: 450, y: 250 }, data: { label: "phrase 01" } },
];
const initialEdges = [{ id: "e1-2", source: "1", target: "2" }];
const proOptions = { hideAttribution: true };
const nodeTypes = {
  [EGraphNodeType.DIALOG_NODE]: DialogNode,
  [EGraphNodeType.PHRASE_NODE]: PhraseNode,
};

export function DialogEditorTestPage(): ReactElement {
  const [nodes, , onNodesChange] = useNodesState(initialNodes);
  const [edges, setEdges, onEdgesChange] = useEdgesState(initialEdges);

  const onConnect = useCallback((params: Edge | Connection) => setEdges((eds) => addEdge(params, eds)), [setEdges]);

  return (
    <Grid
      justifyContent={"center"}
      alignItems={"center"}
      direction={"row"}
      container={true}
      width={"100%"}
      height={"100%"}
      gap={1}
    >
      <Grid width={120} padding={2} height={"100%"} borderRight={"1px solid red"}>
        left menu
      </Grid>

      <Grid flexGrow={1} height={"100%"}>
        <ReactFlow
          proOptions={proOptions}
          nodes={nodes}
          edges={edges}
          nodeTypes={nodeTypes}
          onNodesChange={onNodesChange}
          onEdgesChange={onEdgesChange}
          onConnect={onConnect}
        >
          <Controls position={"bottom-right"} />
          <Background gap={12} size={1} />
        </ReactFlow>
      </Grid>
    </Grid>
  );
}

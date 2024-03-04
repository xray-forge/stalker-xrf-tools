import { Grid } from "@mui/material";
import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";

import { SpawnEditorAlife } from "@/applications/spawn_editor/components/editor/chunks/alife/SpawnEditorAlife";
import { SpawnEditorArtefacts } from "@/applications/spawn_editor/components/editor/chunks/artefacts/SpawnEditorArtefacts";
import { SpawnEditorGraphs } from "@/applications/spawn_editor/components/editor/chunks/graph/SpawnEditorGraphs";
import { SpawnEditorHeader } from "@/applications/spawn_editor/components/editor/chunks/header/SpawnEditorHeader";
import { SpawnEditorPatrols } from "@/applications/spawn_editor/components/editor/chunks/patrol/SpawnEditorPatrols";
import { SpawnEditorMenu } from "@/applications/spawn_editor/components/editor/SpawnEditorMenu";

export function SpawnEditor(): ReactElement {
  return (
    <Grid
      justifyContent={"center"}
      alignItems={"center"}
      direction={"row"}
      container={true}
      flexWrap={"nowrap"}
      width={"100%"}
      height={"100%"}
    >
      <SpawnEditorMenu />

      <Routes>
        <Route path={"/header"} element={<SpawnEditorHeader />} />
        <Route path={"/alife"} element={<SpawnEditorAlife />} />
        <Route path={"/artefacts"} element={<SpawnEditorArtefacts />} />
        <Route path={"/patrols"} element={<SpawnEditorPatrols />} />
        <Route path={"/graph"} element={<SpawnEditorGraphs />} />
        <Route path={"/*"} element={<SpawnEditorHeader />} />
      </Routes>
    </Grid>
  );
}

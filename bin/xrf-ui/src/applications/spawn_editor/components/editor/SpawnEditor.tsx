import { Grid } from "@mui/material";
import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";

import { SpawnEditorAlife } from "@/applications/spawn_editor/components/editor/SpawnEditorAlife";
import { SpawnEditorArtefacts } from "@/applications/spawn_editor/components/editor/SpawnEditorArtefacts";
import { SpawnEditorGeneral } from "@/applications/spawn_editor/components/editor/SpawnEditorGeneral";
import { SpawnEditorGraphs } from "@/applications/spawn_editor/components/editor/SpawnEditorGraphs";
import { SpawnEditorHeader } from "@/applications/spawn_editor/components/editor/SpawnEditorHeader";
import { SpawnEditorMenu } from "@/applications/spawn_editor/components/editor/SpawnEditorMenu";
import { SpawnEditorPatrols } from "@/applications/spawn_editor/components/editor/SpawnEditorPatrols";

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
        <Route path={"/"} element={<SpawnEditorGeneral />} />
        <Route path={"/general"} element={<SpawnEditorGeneral />} />
        <Route path={"/header"} element={<SpawnEditorHeader />} />
        <Route path={"/alife"} element={<SpawnEditorAlife />} />
        <Route path={"/artefacts"} element={<SpawnEditorArtefacts />} />
        <Route path={"/patrols"} element={<SpawnEditorPatrols />} />
        <Route path={"/graph"} element={<SpawnEditorGraphs />} />
      </Routes>
    </Grid>
  );
}

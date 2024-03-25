import { Grid } from "@mui/material";
import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";

import { ExportsViewerConditions } from "@/applications/exports_viewer/components/viewer/exports/ExportsViewerConditions";
import { ExportsViewerDialogs } from "@/applications/exports_viewer/components/viewer/exports/ExportsViewerDialogs";
import { ExportsViewerEffects } from "@/applications/exports_viewer/components/viewer/exports/ExportsViewerEffects";
import { ExportsViewerMenu } from "@/applications/exports_viewer/components/viewer/ExportsViewerMenu";

export function ExportsViewer(): ReactElement {
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
      <ExportsViewerMenu />

      <Routes>
        <Route path={"/conditions"} element={<ExportsViewerConditions />} />
        <Route path={"/effects"} element={<ExportsViewerEffects />} />
        <Route path={"/dialogs"} element={<ExportsViewerDialogs />} />
        <Route path={"/*"} element={<ExportsViewerConditions />} />
      </Routes>
    </Grid>
  );
}

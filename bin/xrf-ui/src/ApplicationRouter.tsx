import { ReactElement } from "react";
import { Route, BrowserRouter as Router, Routes } from "react-router-dom";

import { ArchivesEditorRouter } from "@/applications/archive_editor/ArchivesEditorRouter";
import { ConfigsEditorRouter } from "@/applications/configs_editor/ConfigsEditorRouter";
import { DialogEditor } from "@/applications/dialog_editor/DialogEditor";
import { ExportsViewerRouter } from "@/applications/exports_viewer/ExportsViewerRouter";
import { IconsEditorRouter } from "@/applications/icons_editor/IconsEditorRouter";
import { Root } from "@/applications/Root";
import { SpawnEditorRouter } from "@/applications/spawn_editor/SpawnEditorRouter";
import { NavigationError } from "@/core/components/NavigationError";

export function ApplicationRouter(): ReactElement {
  return (
    <Router>
      <Routes>
        <Route path={"/"} element={<Root />} />
        <Route path={"spawn_editor/*"} element={<SpawnEditorRouter />} />
        <Route path={"archives_editor/*"} element={<ArchivesEditorRouter />} />
        <Route path={"dialog_editor/*"} element={<DialogEditor />} />
        <Route path={"icons_editor/*"} element={<IconsEditorRouter />} />
        <Route path={"configs_editor/*"} element={<ConfigsEditorRouter />} />
        <Route path={"exports_viewer/*"} element={<ExportsViewerRouter />} />
        <Route path={"*"} element={<NavigationError />} />
      </Routes>
    </Router>
  );
}

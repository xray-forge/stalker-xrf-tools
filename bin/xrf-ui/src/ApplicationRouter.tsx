import { ReactElement } from "react";
import { Route, BrowserRouter as Router, Routes } from "react-router-dom";

import { ArchiveEditor } from "@/applications/archive_editor/ArchiveEditor";
import { DialogEditor } from "@/applications/dialog_editor/DialogEditor";
import { IconEditor } from "@/applications/icon_editor/IconEditor";
import { Root } from "@/applications/root/Root";
import { SpawnEditorRouter } from "@/applications/spawn_editor/SpawnEditorRouter";
import { NavigationError } from "@/core/components/NavigationError";

export function ApplicationRouter(): ReactElement {
  return (
    <Router>
      <Routes>
        <Route path={"/"} element={<Root />} />
        <Route path={"spawn_editor/*"} element={<SpawnEditorRouter />} />
        <Route path={"archive_editor/*"} element={<ArchiveEditor />} />
        <Route path={"dialog_editor/*"} element={<DialogEditor />} />
        <Route path={"icon_editor/*"} element={<IconEditor />} />
        <Route path={"*"} element={<NavigationError />} />
      </Routes>
    </Router>
  );
}

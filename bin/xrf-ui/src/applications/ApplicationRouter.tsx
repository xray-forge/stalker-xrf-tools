import { ReactElement } from "react";
import { Route, BrowserRouter as Router, Routes } from "react-router-dom";

import { ArchivesEditorRouter } from "@/applications/archive_editor/ArchivesEditorRouter";
import { ConfigsEditorRouter } from "@/applications/configs_editor/ConfigsEditorRouter";
import { DialogEditorRouter } from "@/applications/dialog_editor/DialogEditorRouter";
import { ExportsEditorRouter } from "@/applications/exports_editor/ExportsEditorRouter";
import { IconsEditorRouter } from "@/applications/icons_editor/IconsEditorRouter";
import { Root } from "@/applications/Root";
import { SpawnEditorRouter } from "@/applications/spawn_editor/SpawnEditorRouter";
import { TranslationsEditorRouter } from "@/applications/translations_editor/TranslationsEditorRouter";
import { NavigationError } from "@/core/components/NavigationError";

export function ApplicationRouter(): ReactElement {
  return (
    <Router>
      <Routes>
        <Route path={"/"} element={<Root />} />
        <Route path={"spawn_editor/*"} element={<SpawnEditorRouter />} />
        <Route path={"archives_editor/*"} element={<ArchivesEditorRouter />} />
        <Route path={"dialog_editor/*"} element={<DialogEditorRouter />} />
        <Route path={"icons_editor/*"} element={<IconsEditorRouter />} />
        <Route path={"configs_editor/*"} element={<ConfigsEditorRouter />} />
        <Route path={"exports_editor/*"} element={<ExportsEditorRouter />} />
        <Route path={"translations_editor/*"} element={<TranslationsEditorRouter />} />
        <Route path={"*"} element={<NavigationError />} />
      </Routes>
    </Router>
  );
}

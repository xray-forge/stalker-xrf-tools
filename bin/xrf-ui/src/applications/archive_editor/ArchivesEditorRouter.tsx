import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";

import { ArchivesEditorNavigatorPage } from "@/applications/archive_editor/pages/ArchivesEditorNavigatorPage";
import { ArchivesEditorUnpackerPage } from "@/applications/archive_editor/pages/ArchivesEditorUnpackerPage";
import { NavigationError } from "@/core/components/NavigationError";

export function ArchivesEditorRouter(): ReactElement {
  return (
    <Routes>
      <Route path={"/"} element={<ArchivesEditorNavigatorPage />} />
      <Route path={"/explorer"} element={<ArchivesEditorNavigatorPage />} />
      <Route path={"/unpacker"} element={<ArchivesEditorUnpackerPage />} />
      <Route path={"*"} element={<NavigationError />} />
    </Routes>
  );
}

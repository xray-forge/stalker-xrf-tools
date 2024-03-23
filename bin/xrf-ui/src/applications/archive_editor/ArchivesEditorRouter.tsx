import { createProvider } from "dreamstate";
import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";

import { ArchivesEditorNavigatorPage } from "@/applications/archive_editor/pages/ArchivesEditorNavigatorPage";
import { ArchivesEditorPage } from "@/applications/archive_editor/pages/ArchivesEditorPage";
import { ArchivesEditorUnpackerPage } from "@/applications/archive_editor/pages/ArchivesEditorUnpackerPage";
import { ArchivesManager } from "@/applications/archive_editor/store/archives";
import { NavigationError } from "@/core/components/NavigationError";

const ArchivesEditorProvider = createProvider([ArchivesManager]);

export function ArchivesEditorRouter(): ReactElement {
  return (
    <ArchivesEditorProvider>
      <Routes>
        <Route path={"/"} element={<ArchivesEditorNavigatorPage />} />
        <Route path={"/editor"} element={<ArchivesEditorPage />} />
        <Route path={"/explorer"} element={<ArchivesEditorNavigatorPage />} />
        <Route path={"/unpacker"} element={<ArchivesEditorUnpackerPage />} />
        <Route path={"*"} element={<NavigationError />} />
      </Routes>
    </ArchivesEditorProvider>
  );
}

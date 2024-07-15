import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";

import { DialogEditorPage } from "@/applications/dialog_editor/pages/DialogEditorPage";
import { DialogEditorTestPage } from "@/applications/dialog_editor/pages/DialogEditorTestPage";
import { NavigationError } from "@/core/components/NavigationError";

export function DialogEditorRouter(): ReactElement {
  return (
    <Routes>
      <Route path={"/todo"} element={<DialogEditorTestPage />} />
      <Route path={"/"} element={<DialogEditorPage />} />
      <Route path={"*"} element={<NavigationError />} />
    </Routes>
  );
}

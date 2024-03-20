import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";

import { ConfigsEditorExplorerPage } from "@/applications/configs_editor/pages/ConfigsEditorExplorerPage";
import { ConfigsEditorFormatterPage } from "@/applications/configs_editor/pages/ConfigsEditorFormatterPage";
import { ConfigsEditorNavigatorPage } from "@/applications/configs_editor/pages/ConfigsEditorNavigatorPage";
import { ConfigsEditorVerifierPage } from "@/applications/configs_editor/pages/ConfigsEditorVerifierPage";
import { NavigationError } from "@/core/components/NavigationError";

export function ConfigsEditorRouter(): ReactElement {
  return (
    <Routes>
      <Route path={"/"} element={<ConfigsEditorNavigatorPage />} />
      <Route path={"/explorer"} element={<ConfigsEditorExplorerPage />} />
      <Route path={"/verifier"} element={<ConfigsEditorVerifierPage />} />
      <Route path={"/formatter"} element={<ConfigsEditorFormatterPage />} />
      <Route path={"*"} element={<NavigationError />} />
    </Routes>
  );
}

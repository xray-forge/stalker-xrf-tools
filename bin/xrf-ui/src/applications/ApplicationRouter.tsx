import { ComponentType, lazy, ReactElement, Suspense } from "react";
import { Route, BrowserRouter as Router, Routes } from "react-router-dom";

import { Root } from "@/applications/Root";
import { ApplicationLoader } from "@/core/components/ApplicationLoader";
import { NavigationError } from "@/core/components/NavigationError";
import { ApplicationShell } from "@/core/components/shell/ApplicationShell";

/**
 * Editor routers are fetched on demand.
 */
function lazyRouter(loader: () => Promise<Record<string, ComponentType>>, name: string): ComponentType {
  return lazy(() => loader().then((module) => ({ default: module[name] })));
}

const ArchivesEditorRouter: ComponentType = lazyRouter(
  () => import("@/applications/archive_editor/ArchivesEditorRouter"),
  "ArchivesEditorRouter"
);
const ConfigsEditorRouter: ComponentType = lazyRouter(
  () => import("@/applications/configs_editor/ConfigsEditorRouter"),
  "ConfigsEditorRouter"
);
const DialogEditorRouter: ComponentType = lazyRouter(
  () => import("@/applications/dialog_editor/DialogEditorRouter"),
  "DialogEditorRouter"
);
const ExportsEditorRouter: ComponentType = lazyRouter(
  () => import("@/applications/exports_editor/ExportsEditorRouter"),
  "ExportsEditorRouter"
);
const IconsEditorRouter: ComponentType = lazyRouter(
  () => import("@/applications/icons_editor/IconsEditorRouter"),
  "IconsEditorRouter"
);
const SpawnEditorRouter: ComponentType = lazyRouter(
  () => import("@/applications/spawn_editor/SpawnEditorRouter"),
  "SpawnEditorRouter"
);
const TranslationsEditorRouter: ComponentType = lazyRouter(
  () => import("@/applications/translations_editor/TranslationsEditorRouter"),
  "TranslationsEditorRouter"
);
const VisualsEditorRouter: ComponentType = lazyRouter(
  () => import("@/applications/visuals_editor/VisualsEditorRouter"),
  "VisualsEditorRouter"
);

export function ApplicationRouter(): ReactElement {
  return (
    <Router>
      <ApplicationShell>
        <Suspense fallback={<ApplicationLoader />}>
          <Routes>
            <Route path={"/"} element={<Root />} />
            <Route path={"spawn_editor/*"} element={<SpawnEditorRouter />} />
            <Route path={"archives_editor/*"} element={<ArchivesEditorRouter />} />
            <Route path={"dialog_editor/*"} element={<DialogEditorRouter />} />
            <Route path={"icons_editor/*"} element={<IconsEditorRouter />} />
            <Route path={"configs_editor/*"} element={<ConfigsEditorRouter />} />
            <Route path={"exports_editor/*"} element={<ExportsEditorRouter />} />
            <Route path={"translations_editor/*"} element={<TranslationsEditorRouter />} />
            <Route path={"visuals_editor/*"} element={<VisualsEditorRouter />} />
            <Route path={"*"} element={<NavigationError />} />
          </Routes>
        </Suspense>
      </ApplicationShell>
    </Router>
  );
}

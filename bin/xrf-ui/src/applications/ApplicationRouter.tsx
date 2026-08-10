import { ComponentType, lazy, ReactElement, Suspense } from "react";
import { Route, BrowserRouter as Router, Routes, useLocation } from "react-router-dom";

import { Root } from "@/applications/Root";
import { ApplicationLoader } from "@/core/components/ApplicationLoader";
import { NavigationError } from "@/core/components/NavigationError";
import { ApplicationShell } from "@/core/components/shell/ApplicationShell";
import { findApplicationTool } from "@/core/components/shell/applicationTools";

/**
 * Editor routers are fetched on demand.
 */
function lazyRouter(loader: () => Promise<Record<string, ComponentType>>, name: string): ComponentType {
  return lazy(() => loader().then((module) => ({ default: module[name] })));
}

const ArchivesEditorRouter: ComponentType = lazyRouter(
  () => import("@/applications/archive-editor"),
  "ArchivesEditorRouter"
);
const ConfigsEditorRouter: ComponentType = lazyRouter(
  () => import("@/applications/configs-editor"),
  "ConfigsEditorRouter"
);
const DialogEditorRouter: ComponentType = lazyRouter(
  () => import("@/applications/dialog-editor"),
  "DialogEditorRouter"
);
const ExportsEditorRouter: ComponentType = lazyRouter(
  () => import("@/applications/exports-editor"),
  "ExportsEditorRouter"
);
const IconsEditorRouter: ComponentType = lazyRouter(() => import("@/applications/icons-editor"), "IconsEditorRouter");

const SpawnEditorRouter: ComponentType = lazyRouter(() => import("@/applications/spawn-editor"), "SpawnEditorRouter");

const TranslationsEditorRouter: ComponentType = lazyRouter(
  () => import("@/applications/translations-editor"),
  "TranslationsEditorRouter"
);
const VisualsEditorRouter: ComponentType = lazyRouter(
  () => import("@/applications/visuals-editor"),
  "VisualsEditorRouter"
);

/**
 * The routes, inside a suspense boundary keyed by the tool that owns the route.
 */
function ApplicationRoutes(): ReactElement {
  const { pathname } = useLocation();

  return (
    <Suspense key={findApplicationTool(pathname)?.path ?? "root"} fallback={<ApplicationLoader />}>
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
  );
}

export function ApplicationRouter(): ReactElement {
  return (
    <Router>
      <ApplicationShell>
        <ApplicationRoutes />
      </ApplicationShell>
    </Router>
  );
}

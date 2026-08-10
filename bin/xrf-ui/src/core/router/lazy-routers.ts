import { ComponentType, lazy } from "react";

export interface IApplicationRoute {
  /** Matches the owning entry in `APPLICATION_TOOLS`, so the rail can never point at a missing route. */
  path: string;
  Component: ComponentType;
}

/**
 * Every editor that is fetched on demand, paired with the route it answers to.
 */
export const APPLICATION_ROUTES: Array<IApplicationRoute> = [
  {
    path: "/archives-editor",
    Component: lazy(() => import("@/applications/archive-editor").then((it) => ({ default: it.ArchivesEditorRouter }))),
  },
  {
    path: "/configs-editor",
    Component: lazy(() => import("@/applications/configs-editor").then((it) => ({ default: it.ConfigsEditorRouter }))),
  },
  {
    path: "/dialog-editor",
    Component: lazy(() => import("@/applications/dialog-editor").then((it) => ({ default: it.DialogEditorRouter }))),
  },
  {
    path: "/exports-editor",
    Component: lazy(() => import("@/applications/exports-editor").then((it) => ({ default: it.ExportsEditorRouter }))),
  },
  {
    path: "/icons-editor",
    Component: lazy(() => import("@/applications/icons-editor").then((it) => ({ default: it.IconsEditorRouter }))),
  },
  {
    path: "/spawn-editor",
    Component: lazy(() => import("@/applications/spawn-editor").then((it) => ({ default: it.SpawnEditorRouter }))),
  },
  {
    path: "/translations-editor",
    Component: lazy(() =>
      import("@/applications/translations-editor").then((it) => ({ default: it.TranslationsEditorRouter }))
    ),
  },
  {
    path: "/visuals-editor",
    Component: lazy(() => import("@/applications/visuals-editor").then((it) => ({ default: it.VisualsEditorRouter }))),
  },
];

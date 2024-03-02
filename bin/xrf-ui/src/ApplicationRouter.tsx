import { ReactElement, useMemo } from "react";
import { createBrowserRouter, RouterProvider } from "react-router-dom";

import { ArchiveEditor } from "@/applications/archive_editor/ArchiveEditor";
import { DialogEditor } from "@/applications/dialog_editor/DialogEditor";
import { IconEditor } from "@/applications/icon_editor/IconEditor";
import { Root } from "@/applications/root/Root";
import { SpawnEditor } from "@/applications/spawn_editor/SpawnEditor";
import { NavigationError } from "@/core/components/NavigationError";

export function ApplicationRouter(): ReactElement {
  const router = useMemo(
    () =>
      createBrowserRouter([
        {
          path: "/",
          element: <Root />,
        },
        {
          path: "spawn_editor/*",
          element: <SpawnEditor />,
        },
        {
          path: "archive_editor/*",
          element: <ArchiveEditor />,
        },
        {
          path: "dialog_editor/*",
          element: <DialogEditor />,
        },
        {
          path: "icon_editor/*",
          element: <IconEditor />,
        },
        {
          path: "*",
          element: <NavigationError />,
        },
      ]),
    []
  );

  return <RouterProvider router={router} />;
}

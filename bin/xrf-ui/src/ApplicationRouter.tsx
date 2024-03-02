import { ReactElement, useMemo } from "react";
import { createBrowserRouter, RouterProvider } from "react-router-dom";

import { ArchiveEditorToolPage } from "@/pages/ArchiveEditorToolPage";
import { DialogEditorToolPage } from "@/pages/DialogEditorToolPage";
import { RootPage } from "@/pages/RootPage";
import { SpawnFileToolPage } from "@/pages/SpawnFileToolPage";

export function ApplicationRouter(): ReactElement {
  const router = useMemo(
    () =>
      createBrowserRouter([
        {
          path: "/",
          element: <RootPage />,
        },
        {
          path: "spawn_editor",
          element: <SpawnFileToolPage />,
        },
        {
          path: "archive_editor",
          element: <ArchiveEditorToolPage />,
        },
        {
          path: "dialog_editor",
          element: <DialogEditorToolPage />,
        },
      ]),
    []
  );

  return <RouterProvider router={router} />;
}

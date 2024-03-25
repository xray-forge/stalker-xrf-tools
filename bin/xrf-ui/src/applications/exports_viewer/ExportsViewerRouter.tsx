import { createProvider } from "dreamstate";
import { ReactElement } from "react";
import { Route, Routes } from "react-router-dom";

import { ExportsViewerNavigatorPage } from "@/applications/exports_viewer/pages/ExportsViewerNavigatorPage";
import { ExportsViewerPage } from "@/applications/exports_viewer/pages/ExportsViewerPage";
import { ExportsManager } from "@/applications/exports_viewer/store/exports";
import { NavigationError } from "@/core/components/NavigationError";

const ExportsViewerProviders = createProvider([ExportsManager]);

export function ExportsViewerRouter(): ReactElement {
  return (
    <ExportsViewerProviders>
      <Routes>
        <Route path={"/"} element={<ExportsViewerNavigatorPage />} />
        <Route path={"/exports/*"} element={<ExportsViewerPage />} />
        <Route path={"*"} element={<NavigationError />} />
      </Routes>
    </ExportsViewerProviders>
  );
}

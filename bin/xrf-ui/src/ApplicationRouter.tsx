import { ReactElement, Suspense } from "react";
import { Route, Routes, useLocation } from "react-router-dom";

import { APPLICATION_CATALOG } from "@/ApplicationCatalog";
import { ApplicationLauncher } from "@/core/launcher/ApplicationLauncher";
import { IApplicationDescriptor } from "@/core/routing/application";
import { CurrentApplicationProvider } from "@/core/routing/current-application.context";
import { ApplicationShell } from "@/core/shell/ApplicationShell";
import { NavigationError } from "@/core/shell/error/NavigationError";
import { ApplicationLoader } from "@/core/shell/loading/ApplicationLoader";
import { Nullable } from "@/lib/types/general";

/**
 * Maps urls onto applications inside the window chrome that outlives every route.
 *
 * Suspense is keyed by application rather than pathname: pathname would remount on in-application
 * navigation too, tearing down the spawn editor and its loaded file every time you moved between its
 * chunks.
 */
export function ApplicationRouter(): ReactElement {
  const { pathname } = useLocation();

  const application: Nullable<IApplicationDescriptor> = APPLICATION_CATALOG.findApplicationByPath(pathname);

  return (
    <CurrentApplicationProvider application={application}>
      <ApplicationShell>
        <Suspense key={application?.path ?? "root"} fallback={<ApplicationLoader />}>
          <Routes>
            <Route
              path={"/"}
              element={
                <ApplicationLauncher
                  applications={APPLICATION_CATALOG.applications}
                  groups={APPLICATION_CATALOG.groups}
                />
              }
            />

            {APPLICATION_CATALOG.applications.map(({ path, Component }: IApplicationDescriptor) => (
              <Route key={path} path={`${path}/*`} element={<Component />} />
            ))}

            <Route path={"*"} element={<NavigationError />} />
          </Routes>
        </Suspense>
      </ApplicationShell>
    </CurrentApplicationProvider>
  );
}

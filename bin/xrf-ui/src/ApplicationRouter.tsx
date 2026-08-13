import { ReactElement, Suspense } from "react";
import { Route, Routes, useLocation } from "react-router-dom";

import { APPLICATION_CATALOG } from "@/ApplicationCatalog";
import { ApplicationRoot } from "@/ApplicationRoot";
import { IApplicationDescriptor } from "@/core/routing/application";
import { ApplicationShell } from "@/core/shell/ApplicationShell";
import { NavigationError } from "@/core/shell/error/NavigationError";
import { ApplicationLoader } from "@/core/shell/loading/ApplicationLoader";

/**
 * Maps urls onto applications inside the window chrome that outlives every route.
 *
 * Suspense is keyed by application rather than pathname: pathname would remount on in-application
 * navigation too, tearing down the spawn editor and its loaded file every time you moved between its
 * chunks.
 */
export function ApplicationRouter(): ReactElement {
  const { pathname } = useLocation();
  const { applications, findApplication } = APPLICATION_CATALOG;

  return (
    <ApplicationShell>
      <Suspense key={findApplication(pathname)?.path ?? "root"} fallback={<ApplicationLoader />}>
        <Routes>
          <Route path={"/"} element={<ApplicationRoot />} />

          {applications.map(({ path, Component }: IApplicationDescriptor) => (
            <Route key={path} path={`${path}/*`} element={<Component />} />
          ))}

          <Route path={"*"} element={<NavigationError />} />
        </Routes>
      </Suspense>
    </ApplicationShell>
  );
}

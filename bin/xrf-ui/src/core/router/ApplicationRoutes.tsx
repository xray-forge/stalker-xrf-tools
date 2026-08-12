import { ReactElement, Suspense } from "react";
import { Route, Routes, useLocation } from "react-router-dom";

import { Root } from "@/applications/Root";
import { ApplicationLoader } from "@/core/components/ApplicationLoader";
import { NavigationError } from "@/core/components/NavigationError";
import { IApplicationDescriptor } from "@/core/router/application";
import { APPLICATIONS, findApplication } from "@/core/router/applications";
import { useApplicationPreload } from "@/core/router/use-application-preload";

/**
 * Maps urls onto applications, inside a suspense boundary keyed by the application that owns the route.
 *
 * Keyed by application rather than by pathname on purpose: pathname would remount on in-application
 * navigation too, tearing down the spawn editor and its loaded file every time you moved between its
 * chunks.
 */
export function ApplicationRoutes(): ReactElement {
  const { pathname } = useLocation();

  useApplicationPreload();

  return (
    <Suspense key={findApplication(pathname)?.path ?? "root"} fallback={<ApplicationLoader />}>
      <Routes>
        <Route path={"/"} element={<Root />} />

        {APPLICATIONS.map(({ path, Component }: IApplicationDescriptor) => (
          <Route key={path} path={`${path}/*`} element={<Component />} />
        ))}

        <Route path={"*"} element={<NavigationError />} />
      </Routes>
    </Suspense>
  );
}

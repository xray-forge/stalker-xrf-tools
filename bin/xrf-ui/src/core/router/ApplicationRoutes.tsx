import { ReactElement, Suspense } from "react";
import { Route, Routes, useLocation } from "react-router-dom";

import { Root } from "@/applications/Root";
import { APPLICATION_ROUTES, IApplicationRoute } from "@/core/router/lazy-routers";
import { ApplicationLoader } from "@/core/components/ApplicationLoader";
import { NavigationError } from "@/core/components/NavigationError";
import { findApplicationTool } from "@/core/components/shell/applicationTools";

/**
 * Maps urls onto editors, inside a suspense boundary keyed by the tool that owns the route.
 * Keyed by tool rather than by pathname on purpose: pathname would remount on in-editor navigation
 * too, tearing down the spawn editor and its loaded file every time you moved between its chunks.
 */
export function ApplicationRoutes(): ReactElement {
  const { pathname } = useLocation();

  return (
    <Suspense key={findApplicationTool(pathname)?.path ?? "root"} fallback={<ApplicationLoader />}>
      <Routes>
        <Route path={"/"} element={<Root />} />

        {APPLICATION_ROUTES.map(({ path, Component }: IApplicationRoute) => (
          <Route key={path} path={`${path}/*`} element={<Component />} />
        ))}

        <Route path={"*"} element={<NavigationError />} />
      </Routes>
    </Suspense>
  );
}

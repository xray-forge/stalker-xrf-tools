import { ReactElement } from "react";
import { BrowserRouter as Router } from "react-router-dom";

import { ApplicationShell } from "@/core/components/shell/ApplicationShell";
import { ApplicationRoutes } from "@/core/router/ApplicationRoutes";

/**
 * Puts the application inside a router and the window chrome that outlives every route.
 */
export function ApplicationRouter(): ReactElement {
  return (
    <Router>
      <ApplicationShell>
        <ApplicationRoutes />
      </ApplicationShell>
    </Router>
  );
}

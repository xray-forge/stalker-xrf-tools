import { ReactElement } from "react";

import { ApplicationProvider } from "@/applications/ApplicationProvider";
import { ApplicationRouter } from "@/applications/ApplicationRouter";

/**
 * The root container.
 */
export function Application(): ReactElement {
  return (
    <ApplicationProvider>
      <ApplicationRouter />
    </ApplicationProvider>
  );
}

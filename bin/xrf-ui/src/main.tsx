import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { ApplicationProvider } from "@/ApplicationProvider";
import { ApplicationRouter } from "@/ApplicationRouter";
import { ErrorBoundary } from "@/core/error/components/ErrorBoundary";
import { RootCrash } from "@/core/error/components/RootCrash";

createRoot(document.getElementById("root") as HTMLElement).render(
  <StrictMode>
    <ErrorBoundary fallback={RootCrash}>
      <ApplicationProvider>
        <ApplicationRouter />
      </ApplicationProvider>
    </ErrorBoundary>
  </StrictMode>
);

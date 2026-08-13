import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { Application } from "@/applications/Application";
import { ErrorBoundary } from "@/core/error/components/ErrorBoundary";
import { RootCrash } from "@/core/error/components/RootCrash";

createRoot(document.getElementById("root") as HTMLElement).render(
  <StrictMode>
    <ErrorBoundary fallback={RootCrash}>
      <Application />
    </ErrorBoundary>
  </StrictMode>
);

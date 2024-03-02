import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import Application from "@/Application";
import "@/styles.css";

createRoot(document.getElementById("root") as HTMLElement).render(
  <StrictMode>
    <Application />
  </StrictMode>
);

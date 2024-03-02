import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import { Application } from "@/Application";
import "@fontsource/roboto/300.css";
import "@fontsource/roboto/400.css";
import "@fontsource/roboto/500.css";
import "@fontsource/roboto/700.css";

createRoot(document.getElementById("root") as HTMLElement).render(
  <StrictMode>
    <Application />
  </StrictMode>
);

import { ReactElement } from "react";

import { PlannedApplication } from "@/core/shell/editor/PlannedApplication";

export function TranslationsBuilderApplication(): ReactElement {
  return (
    <PlannedApplication description={"Compiles the project's translation sources into per-language string tables."} />
  );
}

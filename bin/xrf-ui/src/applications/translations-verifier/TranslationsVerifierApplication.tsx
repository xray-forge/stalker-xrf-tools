import { ReactElement } from "react";

import { PlannedApplication } from "@/core/shell/editor/PlannedApplication";

export function TranslationsVerifierApplication(): ReactElement {
  return <PlannedApplication description={"Reports translations missing from one or more languages."} />;
}

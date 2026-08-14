import { ReactElement } from "react";

import { PlannedApplication } from "@/core/shell/editor/PlannedApplication";

export function DescriptionIconsEditorApplication(): ReactElement {
  return <PlannedApplication description={"Inspects and edits the icons packed into a description sprite."} />;
}

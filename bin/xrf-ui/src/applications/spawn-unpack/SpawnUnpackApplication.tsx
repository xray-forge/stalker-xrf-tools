import { ReactElement } from "react";

import { SpawnEditorUnpackForm } from "@/applications/spawn-unpack/components/SpawnEditorUnpackForm";

/** Extract a packed spawn file into editable chunks. */
export function SpawnUnpackApplication(): ReactElement {
  return <SpawnEditorUnpackForm />;
}

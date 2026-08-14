import { ReactElement } from "react";

import { SpawnEditorUnpackForm } from "@/applications/spawn-unpacker/components/SpawnEditorUnpackForm";

/** Extract a packed spawn file into editable chunks. */
export function SpawnUnpackerApplication(): ReactElement {
  return <SpawnEditorUnpackForm />;
}

import { ReactElement } from "react";

import { SpawnEditorPackForm } from "@/applications/spawn-pack/components/SpawnEditorPackForm";

/** Build a packed spawn file out of unpacked chunks. */
export function SpawnPackApplication(): ReactElement {
  return <SpawnEditorPackForm />;
}

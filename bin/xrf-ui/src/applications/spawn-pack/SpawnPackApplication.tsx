import { Container, ContainerConfig } from "@wirestate/core";
import { ContainerProvider, useContainer } from "@wirestate/react";
import { ReactElement, useMemo } from "react";

import { SpawnEditorPackForm } from "@/applications/spawn-pack/components/SpawnEditorPackForm";
import { SpawnFileService } from "@/lib/spawn-file";

/**
 * Build a packed spawn file out of unpacked chunks.
 */
export function SpawnPackApplication(): ReactElement {
  const parent: Container = useContainer();
  const config: ContainerConfig = useMemo(() => ({ parent, bindings: [SpawnFileService] }), [parent]);

  return (
    <ContainerProvider config={config}>
      <SpawnEditorPackForm />
    </ContainerProvider>
  );
}

import { Container, ContainerConfig } from "@wirestate/core";
import { ContainerProvider, useContainer } from "@wirestate/react";
import { ReactElement, useMemo } from "react";

import { SpawnEditorUnpackForm } from "@/applications/spawn-unpack/components/SpawnEditorUnpackForm";
import { SpawnFileService } from "@/lib/spawn-file";

/**
 * Extract a packed spawn file into editable chunks.
 */
export function SpawnUnpackApplication(): ReactElement {
  const parent: Container = useContainer();
  const config: ContainerConfig = useMemo(() => ({ parent, bindings: [SpawnFileService] }), [parent]);

  return (
    <ContainerProvider config={config}>
      <SpawnEditorUnpackForm />
    </ContainerProvider>
  );
}

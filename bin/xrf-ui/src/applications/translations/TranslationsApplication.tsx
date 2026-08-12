import { Container, ContainerConfig } from "@wirestate/core";
import { ContainerProvider, useContainer } from "@wirestate/react";
import { ReactElement, useMemo } from "react";

import { TranslationsService } from "@/applications/translations/store/translations";
import { TranslationsApplicationScreen } from "@/applications/translations/TranslationsApplicationScreen";

/**
 * Browse and edit localization tables.
 */
export function TranslationsApplication(): ReactElement {
  const parent: Container = useContainer();
  const config: ContainerConfig = useMemo(() => ({ parent, bindings: [TranslationsService] }), [parent]);

  return (
    <ContainerProvider config={config}>
      <TranslationsApplicationScreen />
    </ContainerProvider>
  );
}

import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { ArchivesEditorOpenForm } from "@/applications/archives-explorer/components/ArchivesEditorOpenForm";
import { ArchivesEditor } from "@/applications/archives-explorer/components/editor/ArchivesEditor";
import { ArchivesService } from "@/applications/archives-explorer/services/archives";
import { ApplicationLoader } from "@/core/shell/loading/ApplicationLoader";

/**
 * Picker until something is open, editor once it is.
 *
 * Separate from the application because a provider cannot consume what it provides.
 */
export function ArchivesExplorerApplication(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);

  if (archivesService.isReady) {
    return archivesService.project.value ? <ArchivesEditor /> : <ArchivesEditorOpenForm />;
  }

  return <ApplicationLoader />;
}

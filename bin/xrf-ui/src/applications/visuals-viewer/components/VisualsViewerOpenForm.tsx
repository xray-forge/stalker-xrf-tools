import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";

import { VisualsService } from "@/applications/visuals-viewer/store/visuals";
import { EApplicationId } from "@/core/routing/application";
import { getExistingProjectLinkedGamePath } from "@/core/settings/lib/path";
import { ProjectService } from "@/core/settings/services/project";
import { PickerForm } from "@/core/shell/editor/PickerForm";
import { PathFormRow } from "@/core/ui/form/PathFormRow";
import { IPathField, usePathField } from "@/core/ui/form/use-path-field";
import { Logger, useLogger } from "@/lib/logging";

interface IVisualsViewerOpenFormProps {
  /**
   * Called once an open attempt has finished, successfully or not.
   *
   * A failed open leaves nothing on screen, so the form stays visible with its error either way; this
   * only dismisses a picker that was reopened over a model.
   */
  onFinished?: () => void;
}

export function VisualsViewerOpenForm({ onFinished }: IVisualsViewerOpenFormProps): ReactElement {
  const visualsService: VisualsService = useInjection(VisualsService);
  const projectService: ProjectService = useInjection(ProjectService);

  const log: Logger = useLogger("visuals");

  const isLoading: boolean = visualsService.visual.isLoading;

  const visual: IPathField = usePathField({
    application: EApplicationId.VISUALS_VIEWER,
    id: "visual",
    title: "Select ogf visual",
    filters: [{ name: "Ogf visual", extensions: ["ogf"] }],
    isDisabled: isLoading,
    seed: async () =>
      projectService.xrfProjectPath ? getExistingProjectLinkedGamePath(projectService.xrfProjectPath) : null,
  });

  const onOpen = useCallback(async () => {
    if (!visual.value) {
      log.info("Cannot open a visual without a path");

      return;
    }

    await visualsService.openFile(visual.value);

    onFinished?.();
  }, [visual.value, log, visualsService, onFinished]);

  return (
    <PickerForm
      isLoading={isLoading}
      title={"Open a game visual"}
      description={"Reads the model and shows its bind pose. Nothing is written."}
      error={visualsService.visual.error ? visualsService.visual.error.message : undefined}
      submitLabel={"Open"}
      isSubmitDisabled={!visual.isValid}
      onSubmit={onOpen}
    >
      <PathFormRow label={"Visual file"} description={"Ogf model to preview"} isDisabled={isLoading} field={visual} />
    </PickerForm>
  );
}

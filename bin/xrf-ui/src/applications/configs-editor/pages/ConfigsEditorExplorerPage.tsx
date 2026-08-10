import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { PickerForm } from "@/core/components/navigation/PickerForm";
import { ProjectService } from "@/core/store/project";
import { PathFormRow } from "@/lib/form/PathFormRow";
import { IPathField, usePathField } from "@/lib/form/use-path-field";
import { getProjectConfigsPath } from "@/lib/xrf-path";

export function ConfigsEditorExplorerPage(): ReactElement {
  const projectService: ProjectService = useInjection(ProjectService);

  const configs: IPathField = usePathField({
    id: "configs.explore.directory",
    title: "Provide path to xrf configs",
    isDirectory: true,
    seed: async () => (projectService.xrfProjectPath ? getProjectConfigsPath(projectService.xrfProjectPath) : null),
  });

  return (
    <PickerForm
      title={"Browse LTX configs"}
      backPath={"/configs_editor"}
      // The explorer itself is not implemented yet; this screen only remembers where it would look.
      submitLabel={"Open"}
      isSubmitDisabled
    >
      <PathFormRow label={"Configs directory"} description={"Directory of LTX files to browse"} field={configs} />
    </PickerForm>
  );
}

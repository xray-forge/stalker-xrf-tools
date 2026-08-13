import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { getProjectConfigsPath } from "@/core/settings/lib/path";
import { ProjectService } from "@/core/settings/services/project";
import { PickerForm } from "@/core/shell/editor/PickerForm";
import { PathFormRow } from "@/core/ui/form/PathFormRow";
import { IPathField, usePathField } from "@/core/ui/form/use-path-field";

export function ConfigsExplorerApplication(): ReactElement {
  const projectService: ProjectService = useInjection(ProjectService);

  const configs: IPathField = usePathField({
    id: "configs.explore.directory",
    title: "Provide path to xrf configs",
    isDirectory: true,
    seed: async () => (projectService.xrfProjectPath ? getProjectConfigsPath(projectService.xrfProjectPath) : null),
  });

  return (
    <PickerForm title={"Browse LTX configs"} submitLabel={"Open"} isSubmitDisabled>
      <PathFormRow label={"Configs directory"} description={"Directory of LTX files to browse"} field={configs} />
    </PickerForm>
  );
}

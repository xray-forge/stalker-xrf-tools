import { useInjection } from "@wirestate/react";
import { ReactElement } from "react";

import { PathFormRow } from "@/core/components/form/PathFormRow";
import { IPathField, usePathField } from "@/core/components/form/use-path-field";
import { PickerForm } from "@/core/components/navigation/PickerForm";
import { getProjectConfigsPath } from "@/core/settings/lib/path";
import { ProjectService } from "@/core/settings/services/project";

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

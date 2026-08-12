import { default as InfoIcon } from "@mui/icons-material/Info";

import { SpawnRowDetailsPanel } from "@/applications/spawn-editor/components/editor/details/SpawnRowDetailsPanel";
import { SpawnFileService } from "@/applications/spawn-editor/store/spawn";
import { IEditorTool } from "@/core/components/shell/EditorToolsContext";

export function createSpawnEditorTools(spawnFileService: SpawnFileService): Array<IEditorTool> {
  return [
    {
      id: "details",
      label: "Row details",
      icon: <InfoIcon />,
      isOpenByDefault: false,
      render: () => <SpawnRowDetailsPanel spawnFileService={spawnFileService} />,
    },
  ];
}

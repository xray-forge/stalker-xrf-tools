import { default as InfoIcon } from "@mui/icons-material/Info";

import { SpawnRowDetailsPanel } from "@/applications/spawn/components/editor/details/SpawnRowDetailsPanel";
import { IEditorPanel } from "@/core/components/shell/panel/context";
import { SpawnFileService } from "@/lib/xrf/spawn-file";

export function createSpawnEditorPanels(spawnFileService: SpawnFileService): Array<IEditorPanel> {
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

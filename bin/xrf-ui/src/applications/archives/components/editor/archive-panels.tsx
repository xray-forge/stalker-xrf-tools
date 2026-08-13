import { default as InfoIcon } from "@mui/icons-material/Info";

import { ArchiveFileDetailsPanel } from "@/applications/archives/components/editor/file-details/ArchiveFileDetailsPanel";
import { ArchivesService } from "@/applications/archives/services/archives";
import { IEditorPanel } from "@/core/shell/panel/context";

export function createArchiveEditorPanels(archivesService: ArchivesService): Array<IEditorPanel> {
  return [
    {
      id: "details",
      label: "File details",
      icon: <InfoIcon />,
      isOpenByDefault: false,
      render: () => <ArchiveFileDetailsPanel archivesService={archivesService} />,
    },
  ];
}

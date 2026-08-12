import { default as InfoIcon } from "@mui/icons-material/Info";

import { ArchiveFileDetailsPanel } from "@/applications/archives/components/editor/file-details/ArchiveFileDetailsPanel";
import { ArchivesService } from "@/applications/archives/store/archives";
import { IEditorPanel } from "@/core/components/shell/panel/EditorPanelsContext";

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

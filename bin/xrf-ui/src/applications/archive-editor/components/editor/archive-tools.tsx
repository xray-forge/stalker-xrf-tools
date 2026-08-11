import { default as InfoIcon } from "@mui/icons-material/Info";

import { ArchiveFileDetailsPanel } from "@/applications/archive-editor/components/editor/file-details/ArchiveFileDetailsPanel";
import { ArchivesService } from "@/applications/archive-editor/store/archives";
import { IEditorTool } from "@/core/components/shell/EditorToolsContext";

export function createArchiveEditorTools(archivesService: ArchivesService): Array<IEditorTool> {
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

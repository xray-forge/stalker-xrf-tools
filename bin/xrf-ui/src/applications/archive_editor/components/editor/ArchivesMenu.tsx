import { default as CloseIcon } from "@mui/icons-material/Close";
import { Box } from "@mui/material";
import { RichTreeView, TreeViewDefaultItemModelProperties } from "@mui/x-tree-view";
import { useInjection } from "@wirestate/react";
import { ReactElement, SyntheticEvent, useCallback, useMemo } from "react";

import { ArchivesService } from "@/applications/archive_editor/store/archives";
import { EditorSideMenu, IEditorSideMenuItem } from "@/core/components/editor/EditorSideMenu";
import { Optional } from "@/core/types/general";
import { parseTree } from "@/lib/archive";

export function ArchivesMenu(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);

  const items: Array<TreeViewDefaultItemModelProperties> = useMemo(
    () => parseTree(Object.values(archivesService.project.value?.files ?? {}), "\\"),
    [archivesService.project.value?.files]
  );

  const onSelectListItem = useCallback(
    (_: Optional<SyntheticEvent>, file: Optional<string>) => {
      if (file) {
        // trim '~/' root
        return archivesService.openArchiveFile(file.slice(2));
      }
    },
    [archivesService]
  );

  const actions: Array<IEditorSideMenuItem> = useMemo(
    () => [
      {
        label: "Close",
        icon: <CloseIcon />,
        isDisabled: archivesService.project.isLoading,
        onClick: archivesService.closeArchivesProject,
      },
    ],
    [archivesService.project.isLoading, archivesService.closeArchivesProject]
  );

  return (
    <EditorSideMenu actions={actions}>
      <Box sx={{ padding: 1 }}>
        <RichTreeView items={items} onSelectedItemsChange={onSelectListItem} />
      </Box>
    </EditorSideMenu>
  );
}

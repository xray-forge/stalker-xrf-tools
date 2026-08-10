import { Box } from "@mui/material";
import { RichTreeView, TreeViewDefaultItemModelProperties } from "@mui/x-tree-view";
import { useInjection } from "@wirestate/react";
import { ReactElement, SyntheticEvent, useCallback, useMemo } from "react";

import { ArchivesService } from "@/applications/archive-editor/store/archives";
import { EditorSideMenu } from "@/core/components/editor/EditorSideMenu";
import { Nullable } from "@/core/types/general";
import { parseTree } from "@/lib/archive";

export function ArchivesMenu(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);

  const items: Array<TreeViewDefaultItemModelProperties> = useMemo(
    () => parseTree(Object.values(archivesService.project.value?.files ?? {}), "\\"),
    [archivesService.project.value?.files]
  );

  const onSelectListItem = useCallback(
    (_: Nullable<SyntheticEvent>, file: Nullable<string>) => {
      if (file) {
        // trim '~/' root
        return archivesService.openArchiveFile(file.slice(2));
      }
    },
    [archivesService]
  );

  return (
    <EditorSideMenu>
      <Box sx={{ padding: 1 }}>
        <RichTreeView items={items} onSelectedItemsChange={onSelectListItem} />
      </Box>
    </EditorSideMenu>
  );
}

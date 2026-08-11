import { default as DescriptionIcon } from "@mui/icons-material/Description";
import { default as FolderIcon } from "@mui/icons-material/Folder";
import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { Box, Typography } from "@mui/material";
import { RichTreeView } from "@mui/x-tree-view";
import { useInjection } from "@wirestate/react";
import { ReactElement, SyntheticEvent, useCallback, useEffect, useMemo, useState } from "react";

import { ArchivesMenuHeader } from "@/applications/archive-editor/components/editor/tree/ArchivesMenuHeader";
import { ArchiveTreeItem } from "@/applications/archive-editor/components/editor/tree/ArchiveTreeItem";
import { ArchivesService } from "@/applications/archive-editor/store/archives";
import { EditorSideMenu } from "@/core/components/editor/EditorSideMenu";
import { Nullable } from "@/core/types/general";
import { filterArchiveTree, IArchiveTreeItem, IFilteredArchiveTree, parseTree } from "@/lib/archive";

const EXPLORER_WIDTH: number = 300;
const FILTER_DEBOUNCE_MS: number = 250;

export function ArchivesMenu(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);

  const [query, setQuery] = useState<string>("");
  const [filterQuery, setFilterQuery] = useState<string>("");
  const [expandedItems, setExpandedItems] = useState<Array<string>>([]);

  const items: Array<IArchiveTreeItem> = useMemo(
    () => parseTree(Object.values(archivesService.project.value?.files ?? {}), "\\"),
    [archivesService.project.value?.files]
  );

  const fileCount: number = Object.keys(archivesService.project.value?.files ?? {}).length;
  const filtered: IFilteredArchiveTree = useMemo(() => filterArchiveTree(items, filterQuery), [filterQuery, items]);
  const visibleExpandedItems: Array<string> = filterQuery ? filtered.expandedItems : expandedItems;
  const selectedItem: Nullable<string> = archivesService.fileDescriptor
    ? `file:${archivesService.fileDescriptor.name}`
    : null;

  const onSelectItem = useCallback(
    (_: Nullable<SyntheticEvent>, itemId: Nullable<string>) => {
      if (!itemId?.startsWith("file:")) {
        return;
      }

      const path: string = itemId.slice("file:".length);
      const descriptor = archivesService.project.value?.files[path];

      if (descriptor) {
        void archivesService.selectArchiveFile(descriptor);
      }
    },
    [archivesService]
  );

  const onClearFilter = useCallback(() => {
    setQuery("");
    setFilterQuery("");
  }, []);

  useEffect(() => {
    const timeoutId: ReturnType<typeof setTimeout> = setTimeout(() => setFilterQuery(query), FILTER_DEBOUNCE_MS);

    return () => clearTimeout(timeoutId);
  }, [query]);

  return (
    <EditorSideMenu
      width={EXPLORER_WIDTH}
      header={
        <ArchivesMenuHeader fileCount={fileCount} query={query} onClear={onClearFilter} onQueryChange={setQuery} />
      }
    >
      {filtered.items.length ? (
        <Box sx={{ padding: 0.5 }}>
          <RichTreeView
            isItemSelectionDisabled={(item: IArchiveTreeItem) => item.kind === "directory"}
            items={filtered.items}
            expandedItems={visibleExpandedItems}
            selectedItems={selectedItem}
            expansionTrigger={"content"}
            slots={{
              item: ArchiveTreeItem,
              collapseIcon: FolderOpenIcon,
              expandIcon: FolderIcon,
              endIcon: DescriptionIcon,
            }}
            onExpandedItemsChange={filterQuery ? undefined : (_, next: Array<string>) => setExpandedItems(next)}
            onSelectedItemsChange={onSelectItem}
          />
        </Box>
      ) : (
        <Box sx={{ padding: 2, textAlign: "center" }}>
          <Typography variant={"body2"} sx={{ color: "text.secondary" }}>
            No files match {filterQuery.trim()}.
          </Typography>
        </Box>
      )}
    </EditorSideMenu>
  );
}

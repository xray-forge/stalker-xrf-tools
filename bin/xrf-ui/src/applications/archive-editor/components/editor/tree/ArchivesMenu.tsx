import { default as DescriptionIcon } from "@mui/icons-material/Description";
import { default as FolderIcon } from "@mui/icons-material/Folder";
import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { Box, Typography } from "@mui/material";
import { RichTreeView } from "@mui/x-tree-view";
import { useInjection } from "@wirestate/react";
import { ReactElement, SyntheticEvent, useCallback, useMemo, useState } from "react";

import { ArchivesMenuHeader } from "@/applications/archive-editor/components/editor/tree/ArchivesMenuHeader";
import { ArchiveTreeItem } from "@/applications/archive-editor/components/editor/tree/ArchiveTreeItem";
import { ArchivesService } from "@/applications/archive-editor/store/archives";
import { EditorSearchResults, IEditorSearchResultRow } from "@/core/components/editor/EditorSearchResults";
import { EditorSideMenu } from "@/core/components/editor/EditorSideMenu";
import { Nullable, Optional } from "@/core/types/general";
import { IArchiveFileDescriptor, IArchiveTreeItem, parseTree } from "@/lib/archive";
import { ISearchResult, IUseRankedSearch, useRankedSearch } from "@/lib/search";

const EXPLORER_WIDTH: number = 300;

export function ArchivesMenu(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);

  const [expandedItems, setExpandedItems] = useState<Array<string>>([]);

  const files: Array<IArchiveFileDescriptor> = useMemo(
    () => Object.values(archivesService.project.value?.files ?? {}),
    [archivesService.project.value?.files]
  );

  const items: Array<IArchiveTreeItem> = useMemo(() => parseTree(files, "\\"), [files]);

  // Selecting again while a read or a write is in flight starts work that the previous one will
  // outlive, and the tree would show a selection whose content is still the old one.
  const isBusy: boolean =
    archivesService.file.isLoading ||
    archivesService.image.isLoading ||
    archivesService.singleFileExtraction.isLoading ||
    archivesService.folderExtraction.isLoading;

  const onSelectDescriptor = useCallback(
    (descriptor: IArchiveFileDescriptor) => {
      void archivesService.selectArchiveFile(descriptor);
    },
    [archivesService]
  );

  const search: IUseRankedSearch<IArchiveFileDescriptor> = useRankedSearch({
    items: files,
    toSearchText: (it) => it.name,
    onSelect: onSelectDescriptor,
  });

  const rows: Array<IEditorSearchResultRow> = useMemo(
    () =>
      search.results.map((result: ISearchResult<IArchiveFileDescriptor>) => {
        const separatorAt: number = result.item.name.lastIndexOf("\\");

        return {
          id: result.item.name,
          label: separatorAt === -1 ? result.item.name : result.item.name.slice(separatorAt + 1),
          description: separatorAt === -1 ? undefined : result.item.name.slice(0, separatorAt),
        };
      }),
    [search.results]
  );

  const selectedItem: Nullable<string> = archivesService.fileDescriptor
    ? `file:${archivesService.fileDescriptor.name}`
    : archivesService.directoryPath !== null
      ? `directory:${archivesService.directoryPath || "~"}`
      : null;

  const onSelectPath = useCallback(
    (path: string) => {
      if (isBusy) {
        return;
      }

      const descriptor: Optional<IArchiveFileDescriptor> = archivesService.project.value?.files[path];

      if (descriptor) {
        onSelectDescriptor(descriptor);
      }
    },
    [archivesService, isBusy, onSelectDescriptor]
  );

  const onSelectItem = useCallback(
    (_: Nullable<SyntheticEvent>, itemId: Nullable<string>) => {
      if (isBusy) {
        return;
      }

      if (itemId?.startsWith("file:")) {
        onSelectPath(itemId.slice("file:".length));
      } else if (itemId?.startsWith("directory:")) {
        const path: string = itemId.slice("directory:".length);

        // The synthetic root node stands for the whole archive, which the backend spells as an empty
        // prefix rather than a literal path.
        archivesService.selectArchiveDirectory(path === "~" ? "" : path);
      }
    },
    [archivesService, isBusy, onSelectPath]
  );

  return (
    <EditorSideMenu
      width={EXPLORER_WIDTH}
      header={
        <ArchivesMenuHeader
          fileCount={files.length}
          query={search.query}
          onClear={search.clear}
          onKeyDown={search.onInputKeyDown}
          onQueryChange={search.setQuery}
        />
      }
    >
      {search.isSearching ? (
        <EditorSearchResults
          ariaLabel={"Archive search results"}
          isDisabled={isBusy}
          isStale={search.isStale}
          emptyLabel={`No files match ${search.query.trim()}.`}
          rows={rows}
          total={search.total}
          activeIndex={search.activeIndex}
          onHoverIndex={search.setActiveIndex}
          onSelect={onSelectPath}
        />
      ) : items.length ? (
        <Box sx={{ padding: 0.5 }}>
          <RichTreeView
            isItemSelectionDisabled={() => isBusy}
            items={items}
            expandedItems={expandedItems}
            selectedItems={selectedItem}
            expansionTrigger={"content"}
            slots={{
              item: ArchiveTreeItem,
              collapseIcon: FolderOpenIcon,
              expandIcon: FolderIcon,
              endIcon: DescriptionIcon,
            }}
            onExpandedItemsChange={(_, next: Array<string>) => setExpandedItems(next)}
            onSelectedItemsChange={onSelectItem}
          />
        </Box>
      ) : (
        <Box sx={{ padding: 2, textAlign: "center" }}>
          <Typography variant={"body2"} sx={{ color: "text.secondary" }}>
            No archive files found.
          </Typography>
        </Box>
      )}
    </EditorSideMenu>
  );
}

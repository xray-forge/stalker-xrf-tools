import { default as DataObjectIcon } from "@mui/icons-material/DataObject";
import { default as FolderIcon } from "@mui/icons-material/Folder";
import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { Box, Typography } from "@mui/material";
import { RichTreeView } from "@mui/x-tree-view";
import { ReactElement, SyntheticEvent, useCallback, useMemo, useState } from "react";

import {
  exportDeclarationItemId,
  exportGroupsToTree,
  getExportSearchText,
  groupExports,
  IExportGroup,
  IExportTreeItem,
} from "@/applications/exports/components/viewer/exports/exports-groups";
import { ExportsMenuHeader } from "@/applications/exports/components/viewer/exports/ExportsMenuHeader";
import { EditorSearchResults, IEditorSearchResultRow } from "@/core/components/editor/EditorSearchResults";
import { EditorSideMenu } from "@/core/components/editor/EditorSideMenu";
import { Nullable } from "@/core/types/general";
import { BaseComponentProps } from "@/lib/dom/element-types";
import { ISearchResult, IUseRankedSearch, useRankedSearch } from "@/lib/search";
import { ExportDescriptor } from "@/lib/xrf/bindings/xray-export";

const DECLARATION_ITEM_PREFIX: string = "declaration:";

export interface IExportsMenuProps extends BaseComponentProps {
  declarations: Array<ExportDescriptor>;
  selectedName: Nullable<string>;
  onSelect: (name: string) => void;
}

export function ExportsMenu({ declarations, selectedName, onSelect }: IExportsMenuProps): ReactElement {
  const [expandedItems, setExpandedItems] = useState<Array<string>>([]);

  const groups: Array<IExportGroup> = useMemo(() => groupExports(declarations), [declarations]);
  const items: Array<IExportTreeItem> = useMemo(() => exportGroupsToTree(groups), [groups]);

  const onSelectDeclaration = useCallback(
    (declaration: ExportDescriptor) => {
      onSelect(declaration.name);
    },
    [onSelect]
  );

  const search: IUseRankedSearch<ExportDescriptor> = useRankedSearch({
    items: declarations,
    toSearchText: (it) => it.name,
    toSecondaryText: getExportSearchText,
    onSelect: onSelectDeclaration,
  });

  const rows: Array<IEditorSearchResultRow> = useMemo(
    () =>
      search.results.map((result: ISearchResult<ExportDescriptor>) => {
        const separatorAt: number = result.item.name.lastIndexOf(".");

        return {
          id: result.item.name,
          label: separatorAt === -1 ? result.item.name : result.item.name.slice(separatorAt + 1),
          description: separatorAt === -1 ? undefined : result.item.name.slice(0, separatorAt),
        };
      }),
    [search.results]
  );

  const selectedItem: Nullable<string> = selectedName ? exportDeclarationItemId(selectedName) : null;

  const onSelectItem = useCallback(
    (_: Nullable<SyntheticEvent>, itemId: Nullable<string>) => {
      if (itemId?.startsWith(DECLARATION_ITEM_PREFIX)) {
        onSelect(itemId.slice(DECLARATION_ITEM_PREFIX.length));
      }
    },
    [onSelect]
  );

  return (
    <EditorSideMenu
      header={
        <ExportsMenuHeader
          exportCount={declarations.length}
          query={search.query}
          onClear={search.clear}
          onKeyDown={search.onInputKeyDown}
          onQueryChange={search.setQuery}
        />
      }
    >
      {search.isSearching ? (
        <EditorSearchResults
          ariaLabel={"Export search results"}
          emptyLabel={`No exports match ${search.query.trim()}.`}
          rows={rows}
          total={search.total}
          activeIndex={search.activeIndex}
          isStale={search.isStale}
          onHoverIndex={search.setActiveIndex}
          onSelect={onSelect}
        />
      ) : items.length ? (
        <Box sx={{ padding: 0.5 }}>
          <RichTreeView
            isItemSelectionDisabled={(item: IExportTreeItem) => item.kind === "group"}
            items={items}
            expandedItems={expandedItems}
            selectedItems={selectedItem}
            expansionTrigger={"content"}
            slots={{
              collapseIcon: FolderOpenIcon,
              expandIcon: FolderIcon,
              endIcon: DataObjectIcon,
            }}
            onExpandedItemsChange={(_, next: Array<string>) => setExpandedItems(next)}
            onSelectedItemsChange={onSelectItem}
          />
        </Box>
      ) : (
        <Box sx={{ padding: 2, textAlign: "center" }}>
          <Typography variant={"body2"} sx={{ color: "text.secondary" }}>
            No externs found.
          </Typography>
        </Box>
      )}
    </EditorSideMenu>
  );
}

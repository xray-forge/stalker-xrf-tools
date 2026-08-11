import { default as DataObjectIcon } from "@mui/icons-material/DataObject";
import { default as FolderIcon } from "@mui/icons-material/Folder";
import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { Box, Typography } from "@mui/material";
import { RichTreeView } from "@mui/x-tree-view";
import { ReactElement, SyntheticEvent, useCallback, useEffect, useMemo, useState } from "react";

import {
  exportDeclarationItemId,
  exportGroupsToTree,
  filterExportGroups,
  groupExports,
  IExportGroup,
  IExportTreeItem,
} from "@/applications/exports-editor/components/viewer/exports/exports-groups";
import { ExportsMenuHeader } from "@/applications/exports-editor/components/viewer/exports/ExportsMenuHeader";
import { EditorSideMenu } from "@/core/components/editor/EditorSideMenu";
import { Nullable } from "@/core/types/general";
import { BaseComponentProps } from "@/lib/dom/element-types";
import { IExportDescriptor } from "@/lib/exports";

const EXPLORER_WIDTH: number = 300;
const FILTER_DEBOUNCE_MS: number = 250;
const DECLARATION_ITEM_PREFIX: string = "declaration:";

export interface IExportsMenuProps extends BaseComponentProps {
  declarations: Array<IExportDescriptor>;
  selectedName: Nullable<string>;
  onSelect: (name: string) => void;
}

export function ExportsMenu({ declarations, selectedName, onSelect }: IExportsMenuProps): ReactElement {
  const [query, setQuery] = useState<string>("");
  const [filterQuery, setFilterQuery] = useState<string>("");
  const [expandedItems, setExpandedItems] = useState<Array<string>>([]);

  const groups: Array<IExportGroup> = useMemo(() => groupExports(declarations), [declarations]);
  const filteredGroups: Array<IExportGroup> = useMemo(
    () => filterExportGroups(groups, filterQuery),
    [filterQuery, groups]
  );

  const items: Array<IExportTreeItem> = useMemo(() => exportGroupsToTree(filteredGroups), [filteredGroups]);
  const visibleExpandedItems: Array<string> = filterQuery
    ? filteredGroups.map((group: IExportGroup) => group.id)
    : expandedItems;

  const selectedItem: Nullable<string> = selectedName ? exportDeclarationItemId(selectedName) : null;

  const onSelectItem = useCallback(
    (_: Nullable<SyntheticEvent>, itemId: Nullable<string>) => {
      if (itemId?.startsWith(DECLARATION_ITEM_PREFIX)) {
        onSelect(itemId.slice(DECLARATION_ITEM_PREFIX.length));
      }
    },
    [onSelect]
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
        <ExportsMenuHeader
          exportCount={declarations.length}
          query={query}
          onClear={onClearFilter}
          onQueryChange={setQuery}
        />
      }
    >
      {items.length ? (
        <Box sx={{ padding: 0.5 }}>
          <RichTreeView
            isItemSelectionDisabled={(item: IExportTreeItem) => item.kind === "group"}
            items={items}
            expandedItems={visibleExpandedItems}
            selectedItems={selectedItem}
            expansionTrigger={"content"}
            slots={{
              collapseIcon: FolderOpenIcon,
              expandIcon: FolderIcon,
              endIcon: DataObjectIcon,
            }}
            onExpandedItemsChange={filterQuery ? undefined : (_, next: Array<string>) => setExpandedItems(next)}
            onSelectedItemsChange={onSelectItem}
          />
        </Box>
      ) : (
        <Box sx={{ padding: 2, textAlign: "center" }}>
          <Typography variant={"body2"} sx={{ color: "text.secondary" }}>
            {declarations.length ? `No exports match ${filterQuery.trim()}.` : "No externs found."}
          </Typography>
        </Box>
      )}
    </EditorSideMenu>
  );
}

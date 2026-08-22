import { Box, Tooltip, Typography } from "@mui/material";
import { styled } from "@mui/material/styles";
import { TreeItem, treeItemClasses, TreeItemProps } from "@mui/x-tree-view";
import { useTreeItemModel } from "@mui/x-tree-view/hooks";
import { ReactElement, Ref } from "react";

import { XrayAsset } from "@/core/bindings/types/xrf-vfs";
import { TREE } from "@/core/theme/tokens";
import { IPathTreeItem } from "@/core/ui/tree/path-tree";
import { Nullable } from "@/lib/types/general";

const StyledTreeItem = styled(TreeItem)(({ theme }) => ({
  [`& .${treeItemClasses.content}`]: {
    minHeight: TREE.rowHeight,
    padding: theme.spacing(0, 0.5),
    borderRadius: theme.shape.borderRadius,
  },
  [`& .${treeItemClasses.iconContainer}`]: {
    width: TREE.iconWidth,
    marginRight: TREE.iconGap,
    "& svg": { fontSize: TREE.iconSize },
  },
  [`& .${treeItemClasses.label}`]: {
    minWidth: 0,
    overflow: "hidden",
    // The size, line height and letter spacing of one semantic step, rather than a rem literal reproducing it: the
    // theme's `fontSize: 13` already scales every variant, so a hardcoded size silently stops following it.
    ...theme.typography.body2,
    textOverflow: "ellipsis",
    whiteSpace: "nowrap",
  },
  [`& .${treeItemClasses.groupTransition}`]: {
    marginLeft: TREE.indent,
  },
}));

type IVisualTreeItemProps = TreeItemProps & { ref?: Ref<HTMLLIElement> };

/**
 * One tree row, marking a visual that came out of an archive.
 */
export function VisualTreeItem({ ref, ...props }: IVisualTreeItemProps): ReactElement {
  const item: Nullable<IPathTreeItem<XrayAsset>> = useTreeItemModel<IPathTreeItem<XrayAsset>>(props.itemId) ?? null;
  const asset: Nullable<XrayAsset> = item?.kind === "file" ? item.payload : null;
  const isArchived: boolean = asset?.container.kind === "archive";

  if (!isArchived) {
    return <StyledTreeItem {...props} ref={ref} />;
  }

  return (
    <StyledTreeItem
      {...props}
      ref={ref}
      label={
        <Box sx={{ display: "flex", alignItems: "center", gap: 0.75, minWidth: 0 }}>
          <Box component={"span"} sx={{ minWidth: 0, overflow: "hidden", textOverflow: "ellipsis" }}>
            {props.label}
          </Box>

          <Tooltip title={"Read from an archive volume"}>
            <Typography
              component={"span"}
              variant={"caption"}
              sx={{ color: "text.secondary", flexShrink: 0, opacity: 0.75 }}
            >
              db
            </Typography>
          </Tooltip>
        </Box>
      }
    />
  );
}

import { styled } from "@mui/material/styles";
import { TreeItem, treeItemClasses } from "@mui/x-tree-view";

import { TREE } from "@/core/theme/tokens";

export const ArchiveTreeItem = styled(TreeItem)(({ theme }) => ({
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
    ...theme.typography.body2,
    textOverflow: "ellipsis",
    whiteSpace: "nowrap",
  },
  [`& .${treeItemClasses.groupTransition}`]: {
    marginLeft: TREE.indent,
  },
}));

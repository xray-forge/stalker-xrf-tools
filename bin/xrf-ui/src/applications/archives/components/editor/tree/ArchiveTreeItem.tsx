import { styled } from "@mui/material/styles";
import { TreeItem, treeItemClasses } from "@mui/x-tree-view";

export const ArchiveTreeItem = styled(TreeItem)(({ theme }) => ({
  [`& .${treeItemClasses.content}`]: {
    minHeight: 28,
    padding: "0 4px",
    borderRadius: theme.shape.borderRadius,
  },
  [`& .${treeItemClasses.iconContainer}`]: {
    width: 18,
    marginRight: 4,
    "& svg": { fontSize: 17 },
  },
  [`& .${treeItemClasses.label}`]: {
    minWidth: 0,
    overflow: "hidden",
    fontSize: "0.8125rem",
    textOverflow: "ellipsis",
    whiteSpace: "nowrap",
  },
  [`& .${treeItemClasses.groupTransition}`]: {
    marginLeft: 14,
  },
}));

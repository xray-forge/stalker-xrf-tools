import { Box, Divider, Drawer, List, ListItem, ListItemButton, ListItemIcon, ListItemText } from "@mui/material";
import { ReactElement, ReactNode } from "react";

export const EDITOR_SIDE_MENU_WIDTH: number = 240;

export interface IEditorSideMenuItem {
  label: string;
  icon?: ReactNode;
  description?: string;
  isSelected?: boolean;
  isDisabled?: boolean;
  onClick?: () => void;
}

export interface IEditorSideMenuProps {
  anchor?: "left" | "right";
  width?: number;
  header?: ReactNode;
  sections?: Array<IEditorSideMenuItem>;
  actions?: Array<IEditorSideMenuItem>;
  children?: ReactNode;
}

function renderItem(item: IEditorSideMenuItem): ReactElement {
  return (
    <ListItem key={item.label} disablePadding>
      <ListItemButton selected={item.isSelected} disabled={item.isDisabled} onClick={item.onClick}>
        {item.icon ? <ListItemIcon sx={{ minWidth: 40 }}>{item.icon}</ListItemIcon> : null}
        <ListItemText primary={item.label} secondary={item.description} />
      </ListItemButton>
    </ListItem>
  );
}

/**
 * Side panel shared by every editor workspace.
 *
 * Same width, same anatomy on both sides: an optional header, a scrolling middle that is either a
 * section list or arbitrary content, and actions pinned to the bottom. Editors used to hand roll this
 * drawer each time and drifted on width, paper positioning and where actions ended up.
 */
export function EditorSideMenu({
  anchor = "left",
  width = EDITOR_SIDE_MENU_WIDTH,
  header,
  sections,
  actions,
  children,
}: IEditorSideMenuProps): ReactElement {
  return (
    <Drawer
      anchor={anchor}
      variant={"permanent"}
      open={true}
      sx={{ width, flexShrink: 0, height: "100%" }}
      slotProps={{ paper: { sx: { position: "relative", width, display: "flex", flexDirection: "column" } } }}
    >
      {header ? <Box sx={{ flexShrink: 0 }}>{header}</Box> : null}

      <Box sx={{ flexGrow: 1, minHeight: 0, overflowY: "auto", backgroundColor: "background.default" }}>
        {sections?.length ? <List disablePadding>{sections.map(renderItem)}</List> : null}
        {children}
      </Box>

      {actions?.length ? (
        <>
          <Divider />
          <List disablePadding sx={{ flexShrink: 0 }}>
            {actions.map(renderItem)}
          </List>
        </>
      ) : null}
    </Drawer>
  );
}

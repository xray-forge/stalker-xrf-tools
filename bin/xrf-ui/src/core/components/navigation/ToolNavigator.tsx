import { Box, Card, Divider, List, ListItemButton, ListItemIcon, ListItemText } from "@mui/material";
import { ReactElement, ReactNode } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { ApplicationNavigatorHeader } from "@/core/components/ApplicationNavigatorHeader";
import { NavigationFooter } from "@/core/components/footer/NavigationFooter";
import { CenteredColumn } from "@/core/components/layout/CenteredColumn";

export interface IToolNavigatorItem {
  label: string;
  description?: string;
  icon?: ReactNode;
  to: string;
  isSecondary?: boolean;
}

export interface IToolNavigatorProps {
  title: string;
  helpLink: string;
  items: Array<IToolNavigatorItem>;
  isWithSettings?: boolean;
}

/**
 * Shared launcher screen used by the root menu and every editor's navigator page.
 */
export function ToolNavigator({ title, helpLink, items, isWithSettings }: IToolNavigatorProps): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  const primaryItems: Array<IToolNavigatorItem> = items.filter((item) => !item.isSecondary);
  const secondaryItems: Array<IToolNavigatorItem> = items.filter((item) => item.isSecondary);

  function renderItem(item: IToolNavigatorItem): ReactElement {
    return (
      <ListItemButton key={item.to + item.label} onClick={() => navigate(item.to, { replace: true })}>
        {item.icon ? <ListItemIcon sx={{ minWidth: 40 }}>{item.icon}</ListItemIcon> : null}
        <ListItemText primary={item.label} secondary={item.description} />
      </ListItemButton>
    );
  }

  return (
    <CenteredColumn>
      <Box sx={{ width: "100%", flexShrink: 0 }}>
        <ApplicationNavigatorHeader title={title} helpLink={helpLink} />
      </Box>

      <Card
        sx={{
          display: "flex",
          flexDirection: "column",
          minWidth: 260,
          maxWidth: 360,
          width: "100%",
          minHeight: 0,
          overflow: "hidden",
        }}
      >
        <List disablePadding sx={{ minHeight: 0, overflowY: "auto" }}>
          {primaryItems.map(renderItem)}
        </List>

        {secondaryItems.length ? (
          <>
            <Divider />
            <List disablePadding sx={{ flexShrink: 0 }}>
              {secondaryItems.map(renderItem)}
            </List>
          </>
        ) : null}
      </Card>

      <Box sx={{ width: "100%", flexShrink: 0 }}>
        <NavigationFooter isWithSettings={isWithSettings} />
      </Box>
    </CenteredColumn>
  );
}

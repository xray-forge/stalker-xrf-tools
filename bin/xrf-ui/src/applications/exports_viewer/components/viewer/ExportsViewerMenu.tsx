import { default as CloseIcon } from "@mui/icons-material/Close";
import { default as LooksIcon3 } from "@mui/icons-material/Looks3";
import { default as LooksIcon1 } from "@mui/icons-material/LooksOne";
import { default as LooksIcon2 } from "@mui/icons-material/LooksTwo";
import { Divider, Drawer, List, ListItem, ListItemButton, ListItemIcon, ListItemText } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement, useCallback, useMemo } from "react";
import { NavigateFunction, redirect, useNavigate } from "react-router-dom";

import { ExportsManager } from "@/applications/exports_viewer/store/exports";

export function ExportsViewerMenu({
  exportsContext: { exportsActions, declarations } = useManager(ExportsManager),
}): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  const sections: Array<[string, ReactElement]> = useMemo(
    () => [
      ["Conditions", <LooksIcon1 />],
      ["Dialogs", <LooksIcon2 />],
      ["Effects", <LooksIcon3 />],
    ],
    []
  );

  const onCloseClicked = useCallback(() => {
    navigate("/exports_viewer", { replace: true });

    return exportsActions.close();
  }, [exportsActions, redirect]);

  return (
    <Drawer variant={"permanent"} open={true} sx={{ height: "100%" }} PaperProps={{ sx: { position: "relative" } }}>
      <List>
        {sections.map(([text, icon]) => (
          <ListItem key={text} disablePadding>
            <ListItemButton onClick={() => navigate(text.toLowerCase(), { replace: true })}>
              <ListItemIcon>{icon}</ListItemIcon>
              <ListItemText primary={text} />
            </ListItemButton>
          </ListItem>
        ))}
      </List>

      <Divider />

      <List>
        <ListItem disablePadding>
          <ListItemButton disabled={declarations.isLoading} onClick={onCloseClicked}>
            <ListItemIcon>
              <CloseIcon />
            </ListItemIcon>
            <ListItemText primary={"Close"} />
          </ListItemButton>
        </ListItem>
      </List>
    </Drawer>
  );
}

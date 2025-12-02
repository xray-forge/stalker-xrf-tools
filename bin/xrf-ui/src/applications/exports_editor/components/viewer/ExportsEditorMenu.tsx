import { default as CloseIcon } from "@mui/icons-material/Close";
import { default as LooksIcon3 } from "@mui/icons-material/Looks3";
import { default as LooksIcon1 } from "@mui/icons-material/LooksOne";
import { default as LooksIcon2 } from "@mui/icons-material/LooksTwo";
import { Divider, Drawer, List, ListItem, ListItemButton, ListItemIcon, ListItemText } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement, useCallback, useMemo } from "react";
import { NavigateFunction, redirect, useNavigate } from "react-router-dom";

import { ExportsManager } from "@/applications/exports_editor/store/exports";

export function ExportsEditorMenu({
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

  const onNavigateClicked = useCallback(
    (to: string) => {
      navigate(`/exports_editor/exports/${to}`, { replace: true });
    },
    [navigate]
  );

  const onCloseClicked = useCallback(() => {
    navigate("/exports_editor", { replace: true });

    return exportsActions.close();
  }, [exportsActions, redirect]);

  return (
    <Drawer
      variant={"permanent"}
      open={true}
      sx={{ height: "100%" }}
      slotProps={{ paper: { sx: { position: "relative" } } }}
    >
      <List disablePadding>
        {sections.map(([text, icon]) => (
          <ListItem key={text} disablePadding>
            <ListItemButton onClick={() => onNavigateClicked(text.toLowerCase())}>
              <ListItemIcon>{icon}</ListItemIcon>
              <ListItemText primary={text} />
            </ListItemButton>
          </ListItem>
        ))}
      </List>

      <Divider />

      <List disablePadding>
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

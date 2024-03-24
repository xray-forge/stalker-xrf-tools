import { default as CloseIcon } from "@mui/icons-material/Close";
import { Button, Drawer, Grid, Typography } from "@mui/material";
import { RichTreeView, TreeViewBaseItem } from "@mui/x-tree-view";
import { useManager } from "dreamstate";
import { ReactElement, useMemo } from "react";

import { ArchivesManager } from "@/applications/archive_editor/store/archives";
import { parseTree } from "@/lib/archive";

export function ArchivesMenu({
  archivesContext: { project: { value: project }, archiveActions } = useManager(ArchivesManager),
}): ReactElement {
  const items: Array<TreeViewBaseItem> = useMemo(
    () => parseTree(Object.values(project?.files ?? {}), "\\"),
    [project?.files]
  );

  return (
    <Drawer
      variant={"permanent"}
      open={true}
      sx={{ height: "100%", width: 320 }}
      PaperProps={{ sx: { position: "relative" } }}
    >
      <Grid padding={1} paddingBottom={0}>
        <Typography variant={"h6"} gutterBottom={false}>
          Files
        </Typography>
      </Grid>

      <Grid padding={1} flexGrow={1} overflow={"auto"}>
        <RichTreeView items={items} />
      </Grid>

      <Grid padding={1} marginTop={1}>
        <Button startIcon={<CloseIcon />} variant={"outlined"} fullWidth={true} onClick={archiveActions.closeProject}>
          Close
        </Button>
      </Grid>
    </Drawer>
  );
}

import { CircularProgress, Divider, Grid, Typography } from "@mui/material";
import { useManager } from "dreamstate";

import { ArchivesManager } from "@/applications/archive_editor/store/archives";

export function ArchivesFileContent({ archivesContext: { file } = useManager(ArchivesManager) }) {
  if (file.isLoading) {
    return (
      <Grid
        container
        flexGrow={1}
        alignItems={"center"}
        justifyContent={"center"}
        height={"100%"}
        overflow={"auto"}
        padding={2}
        width={"auto"}
      >
        <CircularProgress />
      </Grid>
    );
  } else if (file.error) {
    return (
      <Grid
        container
        direction={"column"}
        flexGrow={1}
        height={"100%"}
        overflow={"auto"}
        padding={2}
        width={"auto"}
        wrap={"nowrap"}
      >
        <Typography whiteSpace={"pre-line"}>{String(file.error)}</Typography>
      </Grid>
    );
  } else if (file.value) {
    return (
      <Grid
        container
        direction={"column"}
        flexGrow={1}
        height={"100%"}
        overflow={"auto"}
        padding={2}
        width={"auto"}
        wrap={"nowrap"}
      >
        <Grid>
          <Typography variant={"h5"}>
            {file.value.name} ({(file.value.size / 1024 / 1024).toFixed(3)} MB)
          </Typography>
        </Grid>

        <Grid margin={"8px 0"}>
          <Divider />
        </Grid>

        <Grid>
          <Typography whiteSpace={"pre-wrap"} variant={"body1"} component={"pre"} paragraph>
            {file.value.content}
          </Typography>
        </Grid>
      </Grid>
    );
  }

  return (
    <Grid
      container
      direction={"column"}
      flexGrow={1}
      height={"100%"}
      overflow={"auto"}
      padding={2}
      width={"auto"}
      wrap={"nowrap"}
    >
      none
    </Grid>
  );
}

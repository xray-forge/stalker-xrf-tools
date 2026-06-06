import { Box, CircularProgress, Divider, Grid, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";

import { ArchivesService } from "@/applications/archive_editor/store/archives";
import { bytesToMegabytes } from "@/lib/size";

export function ArchivesFileContent() {
  const { file } = useInjection(ArchivesService);

  if (file.isLoading) {
    return (
      <Grid
        container
        sx={{
          flexGrow: 1,
          alignItems: "center",
          justifyContent: "center",
          height: "100%",
          overflow: "auto",
          padding: 2,
          width: "auto",
        }}
      >
        <CircularProgress />
      </Grid>
    );
  } else if (file.error) {
    return (
      <Box
        sx={{
          display: "flex",
          flexDirection: "column",
          flexGrow: 1,
          height: "100%",
          overflow: "auto",
          padding: 2,
          width: "auto",
          flexWrap: "nowrap",
        }}
      >
        <Typography sx={{ whiteSpace: "pre-line" }}>{String(file.error)}</Typography>
      </Box>
    );
  } else if (file.value) {
    return (
      <Box
        sx={{
          display: "flex",
          flexDirection: "column",
          flexGrow: 1,
          height: "100%",
          overflow: "auto",
          padding: 2,
          width: "auto",
          flexWrap: "nowrap",
        }}
      >
        <Box>
          <Typography variant={"h5"}>
            {file.value.name} ({bytesToMegabytes(file.value.size).toFixed(3)} MB)
          </Typography>
        </Box>

        <Box sx={{ margin: "8px 0" }}>
          <Divider />
        </Box>

        <Box>
          <Typography sx={{ marginBottom: 2, whiteSpace: "pre-wrap" }} variant={"body1"} component={"pre"}>
            {file.value.content}
          </Typography>
        </Box>
      </Box>
    );
  }

  return (
    <Box
      sx={{
        display: "flex",
        flexDirection: "column",
        flexGrow: 1,
        height: "100%",
        overflow: "auto",
        padding: 2,
        width: "auto",
        flexWrap: "nowrap",
      }}
    >
      none
    </Box>
  );
}

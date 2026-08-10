import { Box } from "@mui/material";
import { ReactElement } from "react";

export function TranslationsEditorWorkspace(): ReactElement {
  return (
    <Box
      className={"workspace"}
      sx={{
        display: "flex",
        justifyContent: "center",
        alignItems: "center",
        maxWidth: "100%",
        maxHeight: "100%",
        flexGrow: 1,
        padding: 1,
      }}
    >
      todo
    </Box>
  );
}

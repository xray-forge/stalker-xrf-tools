import { Box, Typography } from "@mui/material";
import { purple } from "@mui/material/colors";
import { ReactElement } from "react";

import { IExportParameterDescriptor } from "@/lib/exports";

export interface IExportViewerParametersProps {
  parameter: IExportParameterDescriptor;
}
export function ExportsEditorParameters({ parameter }: IExportViewerParametersProps): ReactElement {
  return (
    <Box key={parameter.name}>
      <Typography sx={{ display: "inline" }} variant={"body2"} color={purple["500"]}>
        {parameter.name}:{" "}
      </Typography>

      <Typography sx={{ display: "inline" }} variant={"body2"}>
        {parameter.typing}
      </Typography>
    </Box>
  );
}

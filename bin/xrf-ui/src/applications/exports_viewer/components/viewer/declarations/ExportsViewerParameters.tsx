import { Grid, Typography } from "@mui/material";
import { ReactElement } from "react";

import { IExportParameterDescriptor } from "@/lib/exports";

export interface IExportViewerParametersProps {
  parameter: IExportParameterDescriptor;
}

export function ExportsViewerParameters({ parameter }: IExportViewerParametersProps): ReactElement {
  return (
    <Grid key={parameter.name}>
      <Typography display={"inline"} variant={"body2"}>
        {parameter.name}:{" "}
      </Typography>

      <Typography display={"inline"} variant={"body2"} color={"secondary"}>
        {parameter.typing}
      </Typography>
    </Grid>
  );
}

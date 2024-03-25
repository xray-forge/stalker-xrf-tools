import { Card, Divider, Grid, Typography } from "@mui/material";
import { ReactElement } from "react";

import { ExportsViewerParameters } from "@/applications/exports_viewer/components/viewer/declarations/ExportsViewerParameters";
import { IExportDescriptor } from "@/lib/exports";

export interface IExportsViewerDeclarationProps {
  descriptor: IExportDescriptor;
}

export function ExportsViewerDeclaration({ descriptor }: IExportsViewerDeclarationProps): ReactElement {
  return (
    <Grid key={descriptor.name}>
      <Card>
        <Grid padding={1}>
          <Typography variant={"h6"} color={"primary"}>
            {descriptor.name}
          </Typography>

          <Divider sx={{ margin: "4px 0" }} />

          <Grid>
            <Typography variant={"subtitle1"}>Parameters:</Typography>

            {descriptor.parameters.map((parameter) => (
              <ExportsViewerParameters key={parameter.name} parameter={parameter} />
            ))}
          </Grid>
        </Grid>
      </Card>
    </Grid>
  );
}

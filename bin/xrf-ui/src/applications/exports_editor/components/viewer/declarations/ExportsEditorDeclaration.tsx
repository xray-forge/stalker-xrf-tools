import { Card, Divider, Grid, Typography } from "@mui/material";
import { ReactElement } from "react";

import { ExportsEditorParameters } from "@/applications/exports_editor/components/viewer/declarations/ExportsEditorParameters";
import { IExportDescriptor } from "@/lib/exports";

export interface IExportsViewerDeclarationProps {
  descriptor: IExportDescriptor;
}

export function ExportsEditorDeclaration({ descriptor }: IExportsViewerDeclarationProps): ReactElement {
  return (
    <Grid key={descriptor.name} width={"100%"} maxWidth={"100%"} paddingRight={1} item>
      <Card elevation={2}>
        <Grid direction={"column"} padding={1} gap={1} container>
          <Typography variant={"h6"} color={"primary"}>
            {descriptor.name}
          </Typography>

          <Divider sx={{ margin: "4px 0" }} />

          {descriptor.comment ? (
            <Grid>
              <Typography variant={"subtitle1"} color={"secondary"}>
                Description:
              </Typography>
              <Typography variant={"body2"} sx={{ whiteSpace: "pre-wrap" }}>
                {descriptor.comment.replace(/ *\* */g, "").trim()}
              </Typography>
            </Grid>
          ) : null}

          {descriptor.parameters.length ? (
            <Grid>
              <Typography variant={"subtitle1"} color={"secondary"}>
                Types:
              </Typography>

              {descriptor.parameters.map((parameter) => (
                <ExportsEditorParameters key={parameter.name} parameter={parameter} />
              ))}
            </Grid>
          ) : null}

          <Grid>
            <Typography variant={"subtitle1"} color={"secondary"}>
              Location:
            </Typography>
            <Typography variant={"body2"} sx={{ whiteSpace: "pre-wrap" }}>
              {descriptor.filename} ({descriptor.line}:{descriptor.col})
            </Typography>
          </Grid>
        </Grid>
      </Card>
    </Grid>
  );
}

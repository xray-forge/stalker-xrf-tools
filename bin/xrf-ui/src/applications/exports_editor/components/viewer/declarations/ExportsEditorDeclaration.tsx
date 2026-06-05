import { Box, Card, Divider, Typography } from "@mui/material";
import { ReactElement } from "react";

import { ExportsEditorParameters } from "@/applications/exports_editor/components/viewer/declarations/ExportsEditorParameters";
import { IExportDescriptor } from "@/lib/exports";

export interface IExportsViewerDeclarationProps {
  descriptor: IExportDescriptor;
}

export function ExportsEditorDeclaration({ descriptor }: IExportsViewerDeclarationProps): ReactElement {
  return (
    <Box key={descriptor.name} sx={{ width: "100%", maxWidth: "100%", paddingRight: 1 }}>
      <Card elevation={2}>
        <Box sx={{ display: "flex", flexDirection: "column", padding: 1, gap: 1 }}>
          <Typography variant={"h6"} color={"primary"}>
            {descriptor.name}
          </Typography>

          <Divider sx={{ margin: "4px 0" }} />

          {descriptor.comment ? (
            <Box>
              <Typography variant={"subtitle1"} color={"secondary"}>
                Description:
              </Typography>
              <Typography variant={"body2"} sx={{ whiteSpace: "pre-wrap" }}>
                {descriptor.comment.replace(/ *\* */g, "").trim()}
              </Typography>
            </Box>
          ) : null}

          {descriptor.parameters.length ? (
            <Box>
              <Typography variant={"subtitle1"} color={"secondary"}>
                Types:
              </Typography>

              {descriptor.parameters.map((parameter) => (
                <ExportsEditorParameters key={parameter.name} parameter={parameter} />
              ))}
            </Box>
          ) : null}

          <Box>
            <Typography variant={"subtitle1"} color={"secondary"}>
              Location:
            </Typography>
            <Typography variant={"body2"} sx={{ whiteSpace: "pre-wrap" }}>
              {descriptor.filename} ({descriptor.line}:{descriptor.col})
            </Typography>
          </Box>
        </Box>
      </Card>
    </Box>
  );
}

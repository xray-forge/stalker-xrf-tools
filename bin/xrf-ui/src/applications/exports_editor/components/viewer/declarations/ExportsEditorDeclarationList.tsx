import { Box, Grid } from "@mui/material";
import { ReactElement, useState } from "react";

import { ExportsEditorDeclaration } from "@/applications/exports_editor/components/viewer/declarations/ExportsEditorDeclaration";
import { ExportsFilterForm } from "@/applications/exports_editor/components/viewer/declarations/ExportsFilterForm";
import { IExportDescriptor } from "@/lib/exports";

export interface IExportsViewerDeclarationProps {
  descriptors: Array<IExportDescriptor>;
}

export function ExportsEditorDeclarationList({ descriptors }: IExportsViewerDeclarationProps): ReactElement {
  const [filter, setFilter] = useState("");

  return (
    <>
      <Grid container sx={{ marginBottom: 2, justifyContent: "flex-start" }}>
        <ExportsFilterForm onFilterValueChangeDebounced={setFilter} />
      </Grid>

      <Box
        sx={{ display: "flex", flexDirection: "column", flexGrow: 1, gap: 1, flexWrap: "nowrap", overflowY: "auto" }}
      >
        {descriptors
          .filter((descriptor) => {
            return filter ? descriptor.name.includes(filter) || descriptor.comment?.includes(filter) : true;
          })
          .map((descriptor) => (
            <ExportsEditorDeclaration key={descriptor.name} descriptor={descriptor} />
          ))}
      </Box>
    </>
  );
}

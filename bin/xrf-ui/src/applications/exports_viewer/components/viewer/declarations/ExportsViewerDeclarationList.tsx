import { Grid } from "@mui/material";
import { ReactElement, useState } from "react";

import { ExportsFilterForm } from "@/applications/exports_viewer/components/viewer/declarations/ExportsFilterForm";
import { ExportsViewerDeclaration } from "@/applications/exports_viewer/components/viewer/declarations/ExportsViewerDeclaration";
import { IExportDescriptor } from "@/lib/exports";

export interface IExportsViewerDeclarationProps {
  descriptors: Array<IExportDescriptor>;
}

export function ExportsViewerDeclarationList({ descriptors }: IExportsViewerDeclarationProps): ReactElement {
  const [filter, setFilter] = useState("");

  return (
    <>
      <Grid marginBottom={2} justifyContent={"flex-start"} container>
        <ExportsFilterForm onFilterValueChangeDebounced={setFilter} />
      </Grid>

      <Grid direction={"column"} flexGrow={1} gap={1} flexWrap={"nowrap"} sx={{ overflowY: "auto" }} container>
        {descriptors
          .filter((descriptor) => {
            return filter ? descriptor.name.includes(filter) || descriptor.comment?.includes(filter) : true;
          })
          .map((descriptor) => (
            <ExportsViewerDeclaration key={descriptor.name} descriptor={descriptor} />
          ))}
      </Grid>
    </>
  );
}

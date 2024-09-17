import { Box, CircularProgress, Divider, Grid, Tab, Tabs, Typography } from "@mui/material";
import { useManager } from "dreamstate";
import { ReactElement, ReactNode, useLayoutEffect, useMemo } from "react";

import { ExportsEditorDeclarationList } from "@/applications/exports_editor/components/viewer/declarations/ExportsEditorDeclarationList";
import { ExportsManager } from "@/applications/exports_editor/store/exports";
import { IExportDescriptor } from "@/lib/exports";
import { useTabState } from "@/lib/tab";

export function ExportsViewerDialogs({
  exportsContext: { declarations: { isLoading, error, value: declarations } } = useManager(ExportsManager),
}): ReactElement {
  const [activeTab, setActiveTab, onActiveTabChange] = useTabState<string>("");

  const { list, dialogSections, dialogExports } = useMemo(() => {
    const dialogExports: Record<string, Array<IExportDescriptor>> = declarations?.dialogs.reduce(
      (acc, it) => {
        const nameParts: Array<string> = it.name.split(".");
        const key: string = nameParts[0] ?? "general";

        acc[key] = acc[key] ?? [];
        acc[key].push({ ...it, name: nameParts.length ? nameParts.slice(1).join(".") : it.name });

        return acc;
      },
      {} as Record<string, Array<IExportDescriptor>>
    ) ?? {};

    const dialogSections: Array<string> = Object.keys(dialogExports);

    const list: ReactNode = (
      <ExportsEditorDeclarationList descriptors={dialogExports[activeTab] ?? dialogExports[dialogSections[0]]} />
    );

    return { dialogExports, dialogSections, list };
  }, [activeTab, declarations?.dialogs]);

  useLayoutEffect(() => {
    if (!dialogExports[activeTab]) {
      setActiveTab(dialogSections[0] ?? "");
    }
  }, [activeTab, dialogExports, dialogSections]);

  if (isLoading) {
    return (
      <Grid justifyContent={"center"} alignItems={"center"} width={"auto"} height={"100%"} flexGrow={1} container>
        <CircularProgress />
      </Grid>
    );
  }

  if (error || !declarations) {
    return (
      <Grid justifyContent={"center"} alignItems={"center"} width={"auto"} height={"100%"} flexGrow={1} container>
        {error ? String(error) : "No value."}
      </Grid>
    );
  }

  return (
    <Grid
      width={"auto"}
      height={"100%"}
      direction={"column"}
      flexWrap={"nowrap"}
      overflow={"auto"}
      p={2}
      flexGrow={1}
      container
    >
      <Typography variant={"h5"}>Dialogs ({declarations.dialogs.length})</Typography>
      <Divider sx={{ margin: "16px 0" }} />

      <Tabs value={activeTab || dialogSections[0]} onChange={onActiveTabChange}>
        {Object.keys(dialogExports).map((it) => (
          <Tab key={it} value={it} label={it} />
        ))}
      </Tabs>

      <Box marginBottom={2} />

      <Grid direction={"column"} flexGrow={1} gap={1} flexWrap={"nowrap"} sx={{ overflowY: "auto" }} container>
        {list}
      </Grid>
    </Grid>
  );
}

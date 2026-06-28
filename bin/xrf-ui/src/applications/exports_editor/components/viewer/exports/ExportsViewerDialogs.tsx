import { Box, CircularProgress, Divider, Grid, Tab, Tabs, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, ReactNode, useLayoutEffect, useMemo } from "react";

import { ExportsEditorDeclarationList } from "@/applications/exports_editor/components/viewer/declarations/ExportsEditorDeclarationList";
import { ExportsService } from "@/applications/exports_editor/store/exports";
import { IExportDescriptor } from "@/lib/exports";
import { useTabState } from "@/lib/tab";

export function ExportsViewerDialogs(): ReactElement {
  const exportsService: ExportsService = useInjection(ExportsService);

  const [activeTab, setActiveTab, onActiveTabChange] = useTabState<string>("");

  const { list, dialogSections, dialogExports } = useMemo(() => {
    const dialogExports: Record<string, Array<IExportDescriptor>> = exportsService.declarations.value?.dialogs.reduce(
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
  }, [activeTab, exportsService.declarations.value?.dialogs]);

  useLayoutEffect(() => {
    if (!dialogExports[activeTab]) {
      setActiveTab(dialogSections[0] ?? "");
    }
  }, [activeTab, dialogExports, dialogSections, setActiveTab]);

  if (exportsService.declarations.isLoading) {
    return (
      <Grid
        container
        sx={{ justifyContent: "center", alignItems: "center", width: "auto", height: "100%", flexGrow: 1 }}
      >
        <CircularProgress />
      </Grid>
    );
  }

  if (exportsService.declarations.error || !exportsService.declarations.value) {
    return (
      <Grid
        container
        sx={{ justifyContent: "center", alignItems: "center", width: "auto", height: "100%", flexGrow: 1 }}
      >
        {exportsService.declarations.error ? String(exportsService.declarations.error) : "No value."}
      </Grid>
    );
  }

  return (
    <Box
      sx={{
        display: "flex",
        width: "auto",
        height: "100%",
        flexDirection: "column",
        flexWrap: "nowrap",
        overflow: "auto",
        p: 2,
        flexGrow: 1,
      }}
    >
      <Typography variant={"h5"}>Dialogs ({exportsService.declarations.value.dialogs.length})</Typography>
      <Divider sx={{ margin: "16px 0" }} />

      <Tabs value={activeTab || dialogSections[0]} onChange={onActiveTabChange}>
        {Object.keys(dialogExports).map((it) => (
          <Tab key={it} value={it} label={it} />
        ))}
      </Tabs>

      <Box sx={{ marginBottom: 2 }} />

      <Box
        sx={{ display: "flex", flexDirection: "column", flexGrow: 1, gap: 1, flexWrap: "nowrap", overflowY: "auto" }}
      >
        {list}
      </Box>
    </Box>
  );
}

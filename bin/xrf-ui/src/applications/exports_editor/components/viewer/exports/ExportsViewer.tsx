import { Box, CircularProgress, Divider, Grid, Tab, Tabs, Typography } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useLayoutEffect, useMemo } from "react";

import { ExportsEditorDeclarationList } from "@/applications/exports_editor/components/viewer/declarations/ExportsEditorDeclarationList";
import { groupExports, IExportGroup } from "@/applications/exports_editor/components/viewer/exports/exports-groups";
import { ExportsService } from "@/applications/exports_editor/store/exports";
import { Optional } from "@/core/types/general";
import { useTabState } from "@/lib/tab";

export function ExportsViewer(): ReactElement {
  const exportsService: ExportsService = useInjection(ExportsService);

  const [activeTab, setActiveTab, onActiveTabChange] = useTabState<string>("");

  const groups: Array<IExportGroup> = useMemo(
    () => groupExports(exportsService.declarations.value ?? []),
    [exportsService.declarations.value]
  );

  const activeGroup: Optional<IExportGroup> =
    groups.find((group: IExportGroup) => group.id === activeTab) ?? groups[0] ?? null;

  useLayoutEffect(() => {
    if (activeGroup && activeTab !== activeGroup.id) {
      setActiveTab(activeGroup.id);
    }
  }, [activeGroup, activeTab, setActiveTab]);

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

  if (!activeGroup) {
    return (
      <Grid
        container
        sx={{ justifyContent: "center", alignItems: "center", width: "auto", height: "100%", flexGrow: 1 }}
      >
        No externs found.
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
      <Typography variant={"h5"}>Exports ({exportsService.declarations.value.length})</Typography>
      <Divider sx={{ margin: "16px 0" }} />

      <Tabs value={activeGroup.id} variant={"scrollable"} scrollButtons={"auto"} onChange={onActiveTabChange}>
        {groups.map((group: IExportGroup) => (
          <Tab key={group.id} value={group.id} label={`${group.label} (${group.declarations.length})`} />
        ))}
      </Tabs>

      <Box sx={{ marginBottom: 2 }} />
      <ExportsEditorDeclarationList descriptors={activeGroup.declarations} />
    </Box>
  );
}

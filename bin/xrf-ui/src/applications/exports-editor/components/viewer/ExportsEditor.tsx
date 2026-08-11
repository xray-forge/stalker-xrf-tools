import { default as RefreshIcon } from "@mui/icons-material/Refresh";
import { Alert, Box, IconButton, LinearProgress, Tooltip } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useEffect, useMemo, useState } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { groupExports, IExportGroup } from "@/applications/exports-editor/components/viewer/exports/exports-groups";
import { ExportsMenu } from "@/applications/exports-editor/components/viewer/exports/ExportsMenu";
import { ExportsViewer } from "@/applications/exports-editor/components/viewer/exports/ExportsViewer";
import { ExportsService } from "@/applications/exports-editor/store/exports";
import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";
import { useEditorBusy } from "@/core/components/shell/EditorBusyContext";
import { useEditorStatus } from "@/core/components/shell/EditorStatusContext";
import { Nullable } from "@/core/types/general";
import { IExportDescriptor, IExportsProject } from "@/lib/exports";

export function ExportsEditor(): ReactElement {
  const exportsService: ExportsService = useInjection(ExportsService);

  const navigate: NavigateFunction = useNavigate();

  const [selectedName, setSelectedName] = useState<Nullable<string>>(null);
  const [isClosing, setClosing] = useState<boolean>(false);
  const [closeError, setCloseError] = useState<Nullable<string>>(null);

  const project: Nullable<IExportsProject> = exportsService.project.value;
  const declarations: Array<IExportDescriptor> = useMemo(() => project?.declarations ?? [], [project?.declarations]);
  const groups: Array<IExportGroup> = useMemo(() => groupExports(declarations), [declarations]);
  const selectedDeclaration: Nullable<IExportDescriptor> =
    declarations.find((declaration: IExportDescriptor) => declaration.name === selectedName) ?? null;
  const isBusy: boolean = exportsService.project.isLoading || isClosing;

  const onSelect = useCallback((name: string): void => setSelectedName(name), []);

  const onRefresh = useCallback((): void => {
    setCloseError(null);
    void exportsService.refreshExportsProject();
  }, [exportsService]);

  const onClose = useCallback(async (): Promise<void> => {
    setClosing(true);
    setCloseError(null);

    try {
      await exportsService.closeExportsProject();
      navigate("/exports-editor", { replace: true });
    } catch (error: unknown) {
      setCloseError(error instanceof Error ? error.message : String(error));
    } finally {
      setClosing(false);
    }
  }, [exportsService, navigate]);

  useEffect(() => {
    if (selectedName && !selectedDeclaration) {
      setSelectedName(null);
    }
  }, [selectedDeclaration, selectedName]);

  useEditorBusy(isBusy);
  useEditorStatus([
    `${declarations.length} exports`,
    `${groups.length} groups`,
    ...(exportsService.project.isLoading ? ["Refreshing"] : []),
  ]);

  return (
    <EditorLayout
      toolbar={
        <>
          <EditorToolbar
            isBackDisabled={isBusy}
            onBack={() => void onClose()}
            subtitle={
              project?.root ? (
                <Tooltip title={project.root}>
                  <Box component={"span"} className={"monospace"}>
                    {project.root}
                  </Box>
                </Tooltip>
              ) : null
            }
            actions={
              <Tooltip describeChild title={"Refresh exports"}>
                <span>
                  <IconButton color={"inherit"} aria-label={"Refresh exports"} disabled={isBusy} onClick={onRefresh}>
                    <RefreshIcon fontSize={"small"} />
                  </IconButton>
                </span>
              </Tooltip>
            }
          />

          {exportsService.project.isLoading ? <LinearProgress sx={{ height: 2 }} /> : null}

          {exportsService.project.error ? (
            <Alert severity={"error"}>Could not refresh exports: {exportsService.project.error.message}</Alert>
          ) : null}

          {closeError ? (
            <Alert severity={"error"} onClose={() => setCloseError(null)}>
              Could not close exports: {closeError}
            </Alert>
          ) : null}
        </>
      }
      menu={<ExportsMenu declarations={declarations} selectedName={selectedName} onSelect={onSelect} />}
    >
      <ExportsViewer declaration={selectedDeclaration} exportCount={declarations.length} />
    </EditorLayout>
  );
}

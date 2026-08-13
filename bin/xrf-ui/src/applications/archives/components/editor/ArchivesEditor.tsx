import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { Alert, Box, LinearProgress, Tooltip } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useMemo, useState } from "react";

import { ARCHIVE_EDITOR_MONOSPACE_FONT } from "@/applications/archives/components/editor/archive-editor.styles";
import { createArchiveEditorPanels } from "@/applications/archives/components/editor/archive-panels";
import { ArchivesFileContent } from "@/applications/archives/components/editor/preview/ArchivesFileContent";
import { ArchivesMenu } from "@/applications/archives/components/editor/tree/ArchivesMenu";
import { ArchivesService } from "@/applications/archives/store/archives";
import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";
import { useEditorBusy } from "@/core/components/shell/EditorBusyContext";
import { useEditorStatus } from "@/core/components/shell/EditorStatusContext";
import { IEditorPanel, useEditorPanels } from "@/core/components/shell/panel/context";
import { Nullable } from "@/core/types/general";
import { formatBytes } from "@/lib/size";
import { ArchiveProject } from "@/lib/xrf/bindings/xray-archive";

export function ArchivesEditor(): ReactElement {
  const archivesService: ArchivesService = useInjection(ArchivesService);

  const [isClosing, setClosing] = useState<boolean>(false);
  const [closeError, setCloseError] = useState<Nullable<string>>(null);

  const project: Nullable<ArchiveProject> = archivesService.project.value;

  const archiveCount: number = project?.archives.length ?? 0;
  const fileCount: number = Object.keys(project?.files ?? {}).length;
  const totalSize: number = project?.sizeReal ?? 0;
  const projectRoot: string = project?.root ?? "";

  const archivePanels: Array<IEditorPanel> = useMemo(
    () => [
      {
        icon: <FolderOpenIcon />,
        id: "archives",
        isOpenByDefault: true,
        label: "Archives",
        render: () => <ArchivesMenu />,
        side: "left",
      },
      ...createArchiveEditorPanels(archivesService),
    ],
    [archivesService]
  );

  // Extraction writes to disk outside the archive. Walking away mid-write left it running against a
  // screen nobody could see, and the only signal it was happening was one button in the content area.
  const isExtracting: boolean = archivesService.operation.isLoading;
  const isBusy: boolean = isClosing || isExtracting;

  const onClose = useCallback(async (): Promise<void> => {
    setClosing(true);
    setCloseError(null);

    try {
      await archivesService.closeArchivesProject();
    } catch (error: unknown) {
      setCloseError(error instanceof Error ? error.message : String(error));
    } finally {
      setClosing(false);
    }
  }, [archivesService]);

  useEditorBusy(isBusy);

  useEditorPanels(archivePanels);

  useEditorStatus([`${archiveCount} archives`, `${fileCount} files`, formatBytes(totalSize)]);

  return (
    <EditorLayout
      toolbar={
        <>
          <EditorToolbar
            subtitle={
              projectRoot ? (
                <Tooltip title={projectRoot}>
                  <Box component={"span"} sx={{ fontFamily: ARCHIVE_EDITOR_MONOSPACE_FONT }}>
                    {projectRoot}
                  </Box>
                </Tooltip>
              ) : null
            }
            onBack={() => void onClose()}
          />

          {isExtracting ? <LinearProgress sx={{ height: 2 }} /> : null}

          {closeError ? (
            <Alert severity={"error"} onClose={() => setCloseError(null)}>
              Could not close archives: {closeError}
            </Alert>
          ) : null}
        </>
      }
    >
      <ArchivesFileContent />
    </EditorLayout>
  );
}

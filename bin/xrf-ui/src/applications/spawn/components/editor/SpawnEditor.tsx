import { default as ViewListIcon } from "@mui/icons-material/ViewList";
import { Box, LinearProgress, Tooltip } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useMemo } from "react";
import { Route, Routes } from "react-router-dom";

import { SpawnEditorAlife } from "@/applications/spawn/components/editor/chunks/alife/SpawnEditorAlife";
import { SpawnEditorArtefacts } from "@/applications/spawn/components/editor/chunks/artefacts/SpawnEditorArtefacts";
import { SpawnEditorGraphs } from "@/applications/spawn/components/editor/chunks/graph/SpawnEditorGraphs";
import { SpawnEditorHeader } from "@/applications/spawn/components/editor/chunks/header/SpawnEditorHeader";
import { SpawnEditorPatrols } from "@/applications/spawn/components/editor/chunks/patrol/SpawnEditorPatrols";
import { createSpawnEditorPanels } from "@/applications/spawn/components/editor/spawn-panels";
import { SpawnEditorActions } from "@/applications/spawn/components/editor/SpawnEditorActions";
import { SpawnEditorMenu } from "@/applications/spawn/components/editor/SpawnEditorMenu";
import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";
import { useEditorBusy } from "@/core/components/shell/EditorBusyContext";
import { useEditorStatus } from "@/core/components/shell/EditorStatusContext";
import { IEditorPanel, useEditorPanels } from "@/core/components/shell/panel/context";
import { Nullable } from "@/core/types/general";
import { SpawnFileService } from "@/lib/spawn-file";
import { SpawnHeaderChunk } from "@/lib/xrf/bindings/xrf-db";

const MONOSPACE_FONT: string = "'Cascadia Mono', 'Consolas', monospace";

export function SpawnEditor(): ReactElement {
  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);

  const header: Nullable<SpawnHeaderChunk> = spawnFileService.header.value;
  const path: Nullable<string> = spawnFileService.path;
  const isWriting: boolean = spawnFileService.operation.isLoading;

  const spawnPanels: Array<IEditorPanel> = useMemo(
    () => [
      {
        icon: <ViewListIcon />,
        id: "chunks",
        isOpenByDefault: true,
        label: "Chunks",
        render: () => <SpawnEditorMenu />,
        side: "left",
      },
      ...createSpawnEditorPanels(spawnFileService),
    ],
    [spawnFileService]
  );

  // Closing does not navigate: the application shows its own picker again once nothing is open.
  const onClose = useCallback(() => spawnFileService.closeSpawnFile(), [spawnFileService]);

  useEditorBusy(spawnFileService.isBusy);

  useEditorPanels(spawnPanels);

  useEditorStatus(
    header ? [`version ${header.version}`, `${header.objectsCount} objects`, `${header.levelsCount} levels`] : []
  );

  return (
    <EditorLayout
      toolbar={
        <>
          <EditorToolbar
            actions={<SpawnEditorActions />}
            subtitle={
              path ? (
                <Tooltip title={path}>
                  <Box component={"span"} sx={{ fontFamily: MONOSPACE_FONT }}>
                    {path}
                  </Box>
                </Tooltip>
              ) : null
            }
            onBack={onClose}
          />

          {isWriting ? <LinearProgress sx={{ height: 2 }} /> : null}
        </>
      }
    >
      <Routes>
        <Route path={"/header"} element={<SpawnEditorHeader />} />
        <Route path={"/alife"} element={<SpawnEditorAlife />} />
        <Route path={"/artefacts"} element={<SpawnEditorArtefacts />} />
        <Route path={"/patrols/*"} element={<SpawnEditorPatrols />} />
        <Route path={"/graph/*"} element={<SpawnEditorGraphs />} />
        <Route path={"/*"} element={<SpawnEditorHeader />} />
      </Routes>
    </EditorLayout>
  );
}

import { Box, LinearProgress, Tooltip } from "@mui/material";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useMemo } from "react";
import { NavigateFunction, Route, Routes, useNavigate } from "react-router-dom";

import { SpawnEditorAlife } from "@/applications/spawn-editor/components/editor/chunks/alife/SpawnEditorAlife";
import { SpawnEditorArtefacts } from "@/applications/spawn-editor/components/editor/chunks/artefacts/SpawnEditorArtefacts";
import { SpawnEditorGraphs } from "@/applications/spawn-editor/components/editor/chunks/graph/SpawnEditorGraphs";
import { SpawnEditorHeader } from "@/applications/spawn-editor/components/editor/chunks/header/SpawnEditorHeader";
import { SpawnEditorPatrols } from "@/applications/spawn-editor/components/editor/chunks/patrol/SpawnEditorPatrols";
import { createSpawnEditorTools } from "@/applications/spawn-editor/components/editor/spawn-tools";
import { SpawnEditorActions } from "@/applications/spawn-editor/components/editor/SpawnEditorActions";
import { SpawnEditorMenu } from "@/applications/spawn-editor/components/editor/SpawnEditorMenu";
import { SpawnFileService } from "@/applications/spawn-editor/store/spawn";
import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";
import { useEditorBusy } from "@/core/components/shell/EditorBusyContext";
import { useEditorStatus } from "@/core/components/shell/EditorStatusContext";
import { IEditorTool, useEditorTools } from "@/core/components/shell/EditorToolsContext";
import { Nullable } from "@/core/types/general";
import { ISpawnFileHeaderChunk } from "@/lib/spawn-file";

const MONOSPACE_FONT: string = "'Cascadia Mono', 'Consolas', monospace";

export function SpawnEditor(): ReactElement {
  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);

  const navigate: NavigateFunction = useNavigate();

  const header: Nullable<ISpawnFileHeaderChunk> = spawnFileService.header.value;
  const path: Nullable<string> = spawnFileService.path;
  const isWriting: boolean = spawnFileService.operation.isLoading;

  const spawnTools: Array<IEditorTool> = useMemo(() => createSpawnEditorTools(spawnFileService), [spawnFileService]);

  const onClose = useCallback(() => {
    navigate("/spawn-editor", { replace: true });

    return spawnFileService.closeSpawnFile();
  }, [navigate, spawnFileService]);

  useEditorBusy(spawnFileService.isBusy);

  useEditorTools(spawnTools);

  useEditorStatus(
    header ? [`version ${header.version}`, `${header.objectsCount} objects`, `${header.levelsCount} levels`] : []
  );

  return (
    <EditorLayout
      toolbar={
        <>
          <EditorToolbar
            actions={<SpawnEditorActions />}
            isBackDisabled={spawnFileService.isBusy}
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
      menu={<SpawnEditorMenu />}
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

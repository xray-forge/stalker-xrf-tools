import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback } from "react";
import { NavigateFunction, Route, Routes, useNavigate } from "react-router-dom";

import { SpawnEditorAlife } from "@/applications/spawn-editor/components/editor/chunks/alife/SpawnEditorAlife";
import { SpawnEditorArtefacts } from "@/applications/spawn-editor/components/editor/chunks/artefacts/SpawnEditorArtefacts";
import { SpawnEditorGraphs } from "@/applications/spawn-editor/components/editor/chunks/graph/SpawnEditorGraphs";
import { SpawnEditorHeader } from "@/applications/spawn-editor/components/editor/chunks/header/SpawnEditorHeader";
import { SpawnEditorPatrols } from "@/applications/spawn-editor/components/editor/chunks/patrol/SpawnEditorPatrols";
import { SpawnEditorMenu } from "@/applications/spawn-editor/components/editor/SpawnEditorMenu";
import { SpawnFileService } from "@/applications/spawn-editor/store/spawn";
import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { EditorToolbar } from "@/core/components/editor/EditorToolbar";
import { useEditorStatus } from "@/core/components/shell/EditorStatusContext";

export function SpawnEditor(): ReactElement {
  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);

  const navigate: NavigateFunction = useNavigate();

  const header = spawnFileService.spawnFile.value?.header;

  useEditorStatus(
    header ? [`version ${header.version}`, `${header.objectsCount} objects`, `${header.levelsCount} levels`] : []
  );

  const onClose = useCallback(() => {
    navigate("/spawn-editor", { replace: true });

    return spawnFileService.closeSpawnFile();
  }, [navigate, spawnFileService]);

  return (
    <EditorLayout toolbar={<EditorToolbar onBack={onClose} />} menu={<SpawnEditorMenu />}>
      <Routes>
        <Route path={"/header"} element={<SpawnEditorHeader />} />
        <Route path={"/alife"} element={<SpawnEditorAlife />} />
        <Route path={"/artefacts"} element={<SpawnEditorArtefacts />} />
        <Route path={"/patrols"} element={<SpawnEditorPatrols />} />
        <Route path={"/graph"} element={<SpawnEditorGraphs />} />
        <Route path={"/*"} element={<SpawnEditorHeader />} />
      </Routes>
    </EditorLayout>
  );
}

import { default as AccountTreeIcon } from "@mui/icons-material/AccountTree";
import { default as GroupsIcon } from "@mui/icons-material/Groups";
import { default as ImportExportIcon } from "@mui/icons-material/ImportExport";
import { default as InfoIcon } from "@mui/icons-material/Info";
import { default as RouteIcon } from "@mui/icons-material/Route";
import { default as SaveIcon } from "@mui/icons-material/Save";
import { default as ScienceIcon } from "@mui/icons-material/Science";
import * as dialog from "@tauri-apps/plugin-dialog";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useMemo } from "react";
import { NavigateFunction, useLocation, useNavigate } from "react-router-dom";

import { SpawnFileService } from "@/applications/spawn-editor/store/spawn";
import { EditorSideMenu, IEditorSideMenuItem } from "@/core/components/editor/EditorSideMenu";
import { Optional } from "@/core/types/general";

export function SpawnEditorMenu(): ReactElement {
  const spawnFileService: SpawnFileService = useInjection(SpawnFileService);

  const navigate: NavigateFunction = useNavigate();
  const { pathname } = useLocation();

  const onSaveClicked = useCallback(async () => {
    const path: Optional<string> = await dialog.save({
      title: "Save spawn file",
      filters: [{ name: "spawn", extensions: ["spawn"] }],
    });

    if (path) {
      await spawnFileService.saveSpawnFile(path);
    }
  }, [spawnFileService]);

  const onExportClicked = useCallback(async () => {
    const path: Optional<string> = (await dialog.open({
      title: "Export spawn file",
      directory: true,
    })) as Optional<string>;

    if (path) {
      await spawnFileService.exportSpawnFile(path);
    }
  }, [spawnFileService]);

  const sections: Array<IEditorSideMenuItem> = useMemo(
    () =>
      [
        { label: "Header", icon: <InfoIcon />, path: "header" },
        { label: "Alife", icon: <GroupsIcon />, path: "alife" },
        { label: "Artefacts", icon: <ScienceIcon />, path: "artefacts" },
        { label: "Patrols", icon: <RouteIcon />, path: "patrols" },
        { label: "Graph", icon: <AccountTreeIcon />, path: "graph" },
      ].map((it) => ({
        label: it.label,
        icon: it.icon,
        isSelected: pathname.endsWith(`/${it.path}`),
        onClick: () => navigate(`/spawn_editor/editor/${it.path}`, { replace: true }),
      })),
    [navigate, pathname]
  );

  const actions: Array<IEditorSideMenuItem> = useMemo(() => {
    const isLoading: boolean = spawnFileService.spawnFile.isLoading;

    return [
      { label: "Save", icon: <SaveIcon />, isDisabled: isLoading, onClick: onSaveClicked },
      { label: "Export", icon: <ImportExportIcon />, isDisabled: isLoading, onClick: onExportClicked },
    ];
  }, [spawnFileService.spawnFile.isLoading, onSaveClicked, onExportClicked]);

  return <EditorSideMenu sections={sections} actions={actions} />;
}

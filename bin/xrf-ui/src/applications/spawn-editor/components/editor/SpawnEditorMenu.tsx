import { default as AccountTreeIcon } from "@mui/icons-material/AccountTree";
import { default as GroupsIcon } from "@mui/icons-material/Groups";
import { default as InfoIcon } from "@mui/icons-material/Info";
import { default as RouteIcon } from "@mui/icons-material/Route";
import { default as ScienceIcon } from "@mui/icons-material/Science";
import { ReactElement, useMemo } from "react";
import { NavigateFunction, useLocation, useNavigate } from "react-router-dom";

import { EditorSideMenu, IEditorSideMenuItem } from "@/core/components/editor/EditorSideMenu";

export function SpawnEditorMenu(): ReactElement {
  const navigate: NavigateFunction = useNavigate();
  const { pathname } = useLocation();

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
        onClick: () => navigate(`/spawn-editor/editor/${it.path}`, { replace: true }),
      })),
    [navigate, pathname]
  );

  return <EditorSideMenu sections={sections} />;
}

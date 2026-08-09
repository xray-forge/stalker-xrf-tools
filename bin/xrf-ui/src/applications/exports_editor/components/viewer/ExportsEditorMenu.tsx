import { default as BoltIcon } from "@mui/icons-material/Bolt";
import { default as ForumIcon } from "@mui/icons-material/Forum";
import { default as RuleIcon } from "@mui/icons-material/Rule";
import { ReactElement, useMemo } from "react";
import { NavigateFunction, useLocation, useNavigate } from "react-router-dom";

import { EditorSideMenu, IEditorSideMenuItem } from "@/core/components/editor/EditorSideMenu";

export function ExportsEditorMenu(): ReactElement {
  const navigate: NavigateFunction = useNavigate();
  const { pathname } = useLocation();

  const sections: Array<IEditorSideMenuItem> = useMemo(
    () =>
      [
        { label: "Conditions", icon: <RuleIcon />, path: "conditions" },
        { label: "Dialogs", icon: <ForumIcon />, path: "dialogs" },
        { label: "Effects", icon: <BoltIcon />, path: "effects" },
      ].map((it) => ({
        label: it.label,
        icon: it.icon,
        isSelected: pathname.endsWith(`/${it.path}`),
        onClick: () => navigate(`/exports_editor/exports/${it.path}`, { replace: true }),
      })),
    [navigate, pathname]
  );

  return <EditorSideMenu sections={sections} />;
}

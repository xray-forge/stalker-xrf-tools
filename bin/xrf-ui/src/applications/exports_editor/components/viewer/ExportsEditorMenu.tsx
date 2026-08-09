import { default as BoltIcon } from "@mui/icons-material/Bolt";
import { default as CloseIcon } from "@mui/icons-material/Close";
import { default as ForumIcon } from "@mui/icons-material/Forum";
import { default as RuleIcon } from "@mui/icons-material/Rule";
import { useInjection } from "@wirestate/react";
import { ReactElement, useCallback, useMemo } from "react";
import { NavigateFunction, useLocation, useNavigate } from "react-router-dom";

import { ExportsService } from "@/applications/exports_editor/store/exports";
import { EditorSideMenu, IEditorSideMenuItem } from "@/core/components/editor/EditorSideMenu";

export function ExportsEditorMenu(): ReactElement {
  const exportsService: ExportsService = useInjection(ExportsService);

  const navigate: NavigateFunction = useNavigate();
  const { pathname } = useLocation();

  const onCloseClicked = useCallback(() => {
    navigate("/exports_editor", { replace: true });

    return exportsService.closeExports();
  }, [exportsService, navigate]);

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

  const actions: Array<IEditorSideMenuItem> = useMemo(
    () => [
      {
        label: "Close",
        icon: <CloseIcon />,
        isDisabled: exportsService.declarations.isLoading,
        onClick: onCloseClicked,
      },
    ],
    [exportsService.declarations.isLoading, onCloseClicked]
  );

  return <EditorSideMenu sections={sections} actions={actions} />;
}

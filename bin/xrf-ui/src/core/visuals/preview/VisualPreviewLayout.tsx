import { default as AccountTreeIcon } from "@mui/icons-material/AccountTree";
import { ReactElement, ReactNode, useCallback, useMemo, useState } from "react";

import { EditorLayout } from "@/core/shell/editor/EditorLayout";
import { useEditorStatus } from "@/core/shell/EditorStatusContext";
import { useEditorPanels } from "@/core/shell/panel/context";
import { createStubVisualMeshData, IVisualMeshData, IVisualPreviewViewOptions } from "@/core/visuals";
import { VISUAL_EDITOR_PANELS } from "@/core/visuals/preview/visual-panels";
import { VisualPreviewAnimationBar } from "@/core/visuals/preview/VisualPreviewAnimationBar";
import { VisualPreviewToolbar } from "@/core/visuals/preview/VisualPreviewToolbar";
import { VisualPreviewViewport } from "@/core/visuals/preview/VisualPreviewViewport";

const DEFAULT_VIEW_OPTIONS: IVisualPreviewViewOptions = {
  isWireframe: false,
  isGridVisible: true,
  isAxesVisible: true,
};

export interface IVisualPreviewLayoutProps {
  /** Published as a left panel when given. Opening a single visual has nothing to browse. */
  tree?: ReactNode;
}

/**
 * Both entry points - opening a single visual, and browsing a gamedata tree - render the same toolbar,
 * viewport, data panel and animation bar. Only the left panel differs.
 */
export function VisualPreviewLayout({ tree }: IVisualPreviewLayoutProps): ReactElement {
  const [options, setOptions] = useState<IVisualPreviewViewOptions>(DEFAULT_VIEW_OPTIONS);
  const [cameraResetToken, setCameraResetToken] = useState(0);

  const mesh: IVisualMeshData = useMemo(() => createStubVisualMeshData(), []);

  const onResetCamera = useCallback(() => setCameraResetToken((it) => it + 1), []);

  useEditorPanels(
    () =>
      tree
        ? [
            {
              icon: <AccountTreeIcon />,
              id: "project",
              isOpenByDefault: true,
              label: "Project",
              render: () => tree,
              side: "left",
            },
            ...VISUAL_EDITOR_PANELS,
          ]
        : VISUAL_EDITOR_PANELS,
    [tree]
  );

  useEditorStatus([`${mesh.positions.length / 3} vertices`, `${mesh.indices.length / 3} triangles`]);

  return (
    <EditorLayout
      toolbar={<VisualPreviewToolbar options={options} onChangeOptions={setOptions} onResetCamera={onResetCamera} />}
      footer={<VisualPreviewAnimationBar />}
    >
      <VisualPreviewViewport mesh={mesh} options={options} cameraResetToken={cameraResetToken} />
    </EditorLayout>
  );
}

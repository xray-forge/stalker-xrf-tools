import { ReactElement, ReactNode, useCallback, useMemo, useState } from "react";

import { VISUAL_EDITOR_TOOLS } from "@/applications/visuals-editor/components/preview/visual-tools";
import { VisualPreviewAnimationBar } from "@/applications/visuals-editor/components/preview/VisualPreviewAnimationBar";
import { VisualPreviewToolbar } from "@/applications/visuals-editor/components/preview/VisualPreviewToolbar";
import { VisualPreviewViewport } from "@/applications/visuals-editor/components/preview/VisualPreviewViewport";
import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { useEditorStatus } from "@/core/components/shell/EditorStatusContext";
import { useEditorTools } from "@/core/components/shell/EditorToolsContext";
import { createStubVisualMeshData, IVisualMeshData, IVisualPreviewViewOptions } from "@/lib/visuals";

const DEFAULT_VIEW_OPTIONS: IVisualPreviewViewOptions = {
  isWireframe: false,
  isGridVisible: true,
  isAxesVisible: true,
};

export interface IVisualPreviewLayoutProps {
  tree?: ReactNode;
}

/**
 * Both entry points - opening a single visual, and browsing a gamedata tree - render the same toolbar,
 * viewport, data panel and animation bar. Only the left slot differs.
 */
export function VisualPreviewLayout({ tree }: IVisualPreviewLayoutProps): ReactElement {
  const [options, setOptions] = useState<IVisualPreviewViewOptions>(DEFAULT_VIEW_OPTIONS);
  const [cameraResetToken, setCameraResetToken] = useState(0);

  const mesh: IVisualMeshData = useMemo(() => createStubVisualMeshData(), []);

  const onResetCamera = useCallback(() => setCameraResetToken((it) => it + 1), []);

  useEditorTools(VISUAL_EDITOR_TOOLS);

  useEditorStatus([`${mesh.positions.length / 3} vertices`, `${mesh.indices.length / 3} triangles`]);

  return (
    <EditorLayout
      toolbar={<VisualPreviewToolbar options={options} onChangeOptions={setOptions} onResetCamera={onResetCamera} />}
      menu={tree}
      footer={<VisualPreviewAnimationBar />}
    >
      <VisualPreviewViewport mesh={mesh} options={options} cameraResetToken={cameraResetToken} />
    </EditorLayout>
  );
}

import { ReactElement, ReactNode, useCallback, useMemo, useState } from "react";

import { VisualDataPanel } from "@/applications/visuals_editor/components/preview/VisualDataPanel";
import { VisualPreviewAnimationBar } from "@/applications/visuals_editor/components/preview/VisualPreviewAnimationBar";
import { VisualPreviewToolbar } from "@/applications/visuals_editor/components/preview/VisualPreviewToolbar";
import { VisualPreviewViewport } from "@/applications/visuals_editor/components/preview/VisualPreviewViewport";
import { EditorLayout } from "@/core/components/editor/EditorLayout";
import { useEditorStatus } from "@/core/components/shell/EditorStatusContext";
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

  useEditorStatus([`${mesh.positions.length / 3} vertices`, `${mesh.indices.length / 3} triangles`]);

  const onResetCamera = useCallback(() => setCameraResetToken((it) => it + 1), []);

  return (
    <EditorLayout
      toolbar={<VisualPreviewToolbar options={options} onChangeOptions={setOptions} onResetCamera={onResetCamera} />}
      menu={tree}
      aside={<VisualDataPanel />}
      footer={<VisualPreviewAnimationBar />}
    >
      <VisualPreviewViewport mesh={mesh} options={options} cameraResetToken={cameraResetToken} />
    </EditorLayout>
  );
}

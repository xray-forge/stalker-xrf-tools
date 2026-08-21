import { default as AccountTreeIcon } from "@mui/icons-material/AccountTree";
import { ReactElement, ReactNode, useCallback, useMemo, useState } from "react";
import { Texture } from "three";

import { EditorLayout } from "@/core/shell/editor/EditorLayout";
import { useEditorStatus } from "@/core/shell/EditorStatusContext";
import { IEditorPanel, useEditorPanels } from "@/core/shell/panel/context";
import { IVisualPreviewViewOptions } from "@/core/visuals";
import { IVisualModelViews } from "@/core/visuals/lib/visual-views";
import { VisualPreviewAnimationBar } from "@/core/visuals/preview/VisualPreviewAnimationBar";
import { VisualPreviewToolbar } from "@/core/visuals/preview/VisualPreviewToolbar";
import { VisualPreviewViewport } from "@/core/visuals/preview/VisualPreviewViewport";
import { Nullable } from "@/lib/types/general";

const DEFAULT_VIEW_OPTIONS: IVisualPreviewViewOptions = {
  isWireframe: false,
  isGridVisible: true,
  isAxesVisible: true,
  isCheckerVisible: false,
};

export interface IVisualPreviewLayoutProps {
  /** The model on screen, or null while nothing is open. */
  model?: Nullable<IVisualModelViews>;
  /** Shown in the toolbar beside the view toggles, usually where the model came from. */
  subtitle?: string;
  /** Published as a left panel when given. Opening a single visual has nothing to browse. */
  tree?: ReactNode;
  /** Data panels the owning application contributes to the right stripe. */
  panels?: Array<IEditorPanel>;
  /** Loaded textures by submesh index, passed straight through to the viewport. */
  textures?: ReadonlyMap<number, Texture>;
  /** Reopens the picker. Absent while an application has no way to choose a different visual. */
  onOpen?: () => void;
}

/**
 * The shared preview chrome: toolbar, viewport, panel stripe and animation bar.
 *
 * Data comes in as props rather than being read here, so this stays usable by an application that has a
 * backing service and by one that does not.
 */
export function VisualPreviewLayout({
  model = null,
  subtitle,
  tree,
  panels,
  textures,
  onOpen,
}: IVisualPreviewLayoutProps): ReactElement {
  const [options, setOptions] = useState<IVisualPreviewViewOptions>(DEFAULT_VIEW_OPTIONS);
  const [cameraResetToken, setCameraResetToken] = useState(0);

  const onResetCamera = useCallback(() => setCameraResetToken((it) => it + 1), []);

  useEditorPanels(() => {
    const stripe: Array<IEditorPanel> = panels ? [...panels] : [];

    return tree
      ? [
          {
            icon: <AccountTreeIcon />,
            id: "project",
            isOpenByDefault: true,
            label: "Project",
            render: () => tree,
            side: "left",
          },
          ...stripe,
        ]
      : stripe;
  }, [tree, panels]);

  const status: Array<string> = useMemo(
    () =>
      model
        ? [`${model.submeshes.length} submeshes`, `${model.vertexCount} vertices`, `${model.triangleCount} triangles`]
        : ["No visual open"],
    [model]
  );

  useEditorStatus(status);

  return (
    <EditorLayout
      toolbar={
        <VisualPreviewToolbar
          subtitle={subtitle}
          options={options}
          isOpenEnabled={Boolean(onOpen)}
          onChangeOptions={setOptions}
          onResetCamera={onResetCamera}
          onOpen={onOpen}
        />
      }
      footer={<VisualPreviewAnimationBar />}
    >
      <VisualPreviewViewport model={model} options={options} cameraResetToken={cameraResetToken} textures={textures} />
    </EditorLayout>
  );
}

import { Box } from "@mui/material";
import { ReactElement, ReactNode, useCallback, useState } from "react";

import { VisualDataPanel } from "@/applications/visuals_editor/components/preview/VisualDataPanel";
import { VisualPreviewAnimationBar } from "@/applications/visuals_editor/components/preview/VisualPreviewAnimationBar";
import { VisualPreviewToolbar } from "@/applications/visuals_editor/components/preview/VisualPreviewToolbar";
import { VisualPreviewViewport } from "@/applications/visuals_editor/components/preview/VisualPreviewViewport";
import { IVisualPreviewViewOptions } from "@/lib/visuals";

const DEFAULT_VIEW_OPTIONS: IVisualPreviewViewOptions = {
  isWireframe: false,
  isGridVisible: true,
  isAxesVisible: true,
};

export interface IVisualPreviewLayoutProps {
  tree?: ReactNode;
}

/**
 * Editor shell shared by every way of getting a visual on screen.
 *
 * Both entry points - opening a single visual, and browsing a gamedata tree - render the same toolbar,
 * viewport, data panel and animation bar. Only the left slot differs, so adding a mode does not mean
 * rebuilding the surrounding chrome.
 */
export function VisualPreviewLayout({ tree }: IVisualPreviewLayoutProps): ReactElement {
  const [options, setOptions] = useState<IVisualPreviewViewOptions>(DEFAULT_VIEW_OPTIONS);
  const [cameraResetToken, setCameraResetToken] = useState(0);

  const onResetCamera = useCallback(() => setCameraResetToken((it) => it + 1), []);

  return (
    <Box sx={{ display: "flex", flexDirection: "column", width: "100%", height: "100%", flexWrap: "nowrap" }}>
      <VisualPreviewToolbar options={options} onChangeOptions={setOptions} onResetCamera={onResetCamera} />

      <Box sx={{ display: "flex", flexGrow: 1, minHeight: 0, flexWrap: "nowrap" }}>
        {tree}

        <Box sx={{ flexGrow: 1, minWidth: 0 }}>
          <VisualPreviewViewport options={options} cameraResetToken={cameraResetToken} />
        </Box>

        <VisualDataPanel />
      </Box>

      <VisualPreviewAnimationBar />
    </Box>
  );
}

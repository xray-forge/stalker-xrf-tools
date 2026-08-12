import { default as CenterFocusStrongIcon } from "@mui/icons-material/CenterFocusStrong";
import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { default as GridOnIcon } from "@mui/icons-material/GridOn";
import { default as HexagonIcon } from "@mui/icons-material/Hexagon";
import { default as ThreeDRotationIcon } from "@mui/icons-material/ThreeDRotation";
import { default as TuneIcon } from "@mui/icons-material/Tune";
import { Divider, IconButton, Tooltip } from "@mui/material";
import { ReactElement, useCallback } from "react";

import { EditorToolbar } from "@/core/components/editor/EditorToolbar";
import { IVisualPreviewViewOptions } from "@/lib/visuals";

interface IVisualPreviewToolbarProps {
  options: IVisualPreviewViewOptions;
  onChangeOptions: (options: IVisualPreviewViewOptions) => void;
  onResetCamera: () => void;
}

/**
 * View toggles are live and drive the scene. Everything sourcing data - opening a visual, picking a
 * detail level - is present but disabled, because it needs the rust side that does not exist yet.
 */
export function VisualPreviewToolbar({
  options,
  onChangeOptions,
  onResetCamera,
}: IVisualPreviewToolbarProps): ReactElement {
  const onToggleWireframe = useCallback(() => {
    onChangeOptions({ ...options, isWireframe: !options.isWireframe });
  }, [options, onChangeOptions]);

  const onToggleGrid = useCallback(() => {
    onChangeOptions({ ...options, isGridVisible: !options.isGridVisible });
  }, [options, onChangeOptions]);

  const onToggleAxes = useCallback(() => {
    onChangeOptions({ ...options, isAxesVisible: !options.isAxesVisible });
  }, [options, onChangeOptions]);

  return (
    <EditorToolbar
      subtitle={"Stub visual"}
      backPath={"/"}
      actions={
        <>
          <Tooltip title={"Open visual (needs backend)"}>
            <span>
              <IconButton color={"inherit"} disabled>
                <FolderOpenIcon />
              </IconButton>
            </span>
          </Tooltip>

          <Tooltip title={"Detail level (needs backend)"}>
            <span>
              <IconButton color={"inherit"} disabled>
                <TuneIcon />
              </IconButton>
            </span>
          </Tooltip>

          <Divider orientation={"vertical"} flexItem sx={{ marginX: 0.5, marginY: 1 }} />

          <Tooltip title={"Wireframe"}>
            <IconButton color={"inherit"} sx={{ opacity: options.isWireframe ? 1 : 0.45 }} onClick={onToggleWireframe}>
              <HexagonIcon />
            </IconButton>
          </Tooltip>

          <Tooltip title={"Grid"}>
            <IconButton color={"inherit"} sx={{ opacity: options.isGridVisible ? 1 : 0.45 }} onClick={onToggleGrid}>
              <GridOnIcon />
            </IconButton>
          </Tooltip>

          <Tooltip title={"Axes"}>
            <IconButton color={"inherit"} sx={{ opacity: options.isAxesVisible ? 1 : 0.45 }} onClick={onToggleAxes}>
              <ThreeDRotationIcon />
            </IconButton>
          </Tooltip>

          <Tooltip title={"Reset camera"}>
            <IconButton color={"inherit"} onClick={onResetCamera}>
              <CenterFocusStrongIcon />
            </IconButton>
          </Tooltip>
        </>
      }
    />
  );
}

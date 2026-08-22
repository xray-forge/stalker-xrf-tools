import { default as AccountTreeIcon } from "@mui/icons-material/AccountTree";
import { default as CenterFocusStrongIcon } from "@mui/icons-material/CenterFocusStrong";
import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { default as GridOnIcon } from "@mui/icons-material/GridOn";
import { default as HexagonIcon } from "@mui/icons-material/Hexagon";
import { default as TextureIcon } from "@mui/icons-material/Texture";
import { default as ThreeDRotationIcon } from "@mui/icons-material/ThreeDRotation";
import { default as TuneIcon } from "@mui/icons-material/Tune";
import { Divider, IconButton, Tooltip } from "@mui/material";
import { ReactElement, useCallback } from "react";

import { EditorToolbar } from "@/core/shell/editor/EditorToolbar";
import { IVisualPreviewViewOptions } from "@/core/visuals";
import { BaseComponentProps } from "@/lib/dom/element-types";

interface IVisualPreviewToolbarProps extends BaseComponentProps {
  subtitle?: string;
  options: IVisualPreviewViewOptions;
  isOpenEnabled: boolean;
  onChangeOptions: (options: IVisualPreviewViewOptions) => void;
  onResetCamera: () => void;
  onOpen?: () => void;
  onBrowse?: () => void;
}

/**
 * View toggles are live and drive the scene. The detail level picker stays disabled: every detail level
 * is already in the buffer, so it needs a range change rather than a backend, but choosing one is a
 * later phase.
 */
export function VisualPreviewToolbar({
  subtitle,
  options,
  isOpenEnabled,
  onChangeOptions,
  onResetCamera,
  onOpen,
  onBrowse,
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

  const onToggleChecker = useCallback(() => {
    onChangeOptions({ ...options, isCheckerVisible: !options.isCheckerVisible });
  }, [options, onChangeOptions]);

  return (
    <EditorToolbar
      subtitle={subtitle}
      actions={
        <>
          <Tooltip title={isOpenEnabled ? "Open visual" : "Open visual (not available here)"}>
            <span>
              <IconButton color={"inherit"} disabled={!isOpenEnabled} onClick={onOpen} aria-label={"Open visual"}>
                <FolderOpenIcon />
              </IconButton>
            </span>
          </Tooltip>

          {onBrowse ? (
            <Tooltip title={"Browse the folder this model sits in"}>
              <IconButton aria-label={"Browse folder"} color={"inherit"} onClick={onBrowse}>
                <AccountTreeIcon />
              </IconButton>
            </Tooltip>
          ) : null}

          <Tooltip title={"Detail level (not implemented)"} describeChild>
            <span>
              <IconButton color={"inherit"} disabled aria-label={"Detail level"}>
                <TuneIcon />
              </IconButton>
            </span>
          </Tooltip>

          <Divider orientation={"vertical"} flexItem sx={{ marginX: 0.5, marginY: 1 }} />

          <Tooltip title={"Wireframe"}>
            <IconButton
              color={"inherit"}
              sx={{ opacity: options.isWireframe ? 1 : 0.45 }}
              aria-label={"Wireframe"}
              onClick={onToggleWireframe}
            >
              <HexagonIcon />
            </IconButton>
          </Tooltip>

          <Tooltip title={"Uv checkerboard"}>
            <IconButton
              color={"inherit"}
              sx={{ opacity: options.isCheckerVisible ? 1 : 0.45 }}
              aria-label={"Uv checkerboard"}
              onClick={onToggleChecker}
            >
              <TextureIcon />
            </IconButton>
          </Tooltip>

          <Tooltip title={"Grid"}>
            <IconButton
              color={"inherit"}
              sx={{ opacity: options.isGridVisible ? 1 : 0.45 }}
              aria-label={"Grid"}
              onClick={onToggleGrid}
            >
              <GridOnIcon />
            </IconButton>
          </Tooltip>

          <Tooltip title={"Axes"}>
            <IconButton
              color={"inherit"}
              sx={{ opacity: options.isAxesVisible ? 1 : 0.45 }}
              aria-label={"Axes"}
              onClick={onToggleAxes}
            >
              <ThreeDRotationIcon />
            </IconButton>
          </Tooltip>

          <Tooltip title={"Reset camera"}>
            <IconButton color={"inherit"} aria-label={"Reset camera"} onClick={onResetCamera}>
              <CenterFocusStrongIcon />
            </IconButton>
          </Tooltip>
        </>
      }
    />
  );
}

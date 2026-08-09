import { default as ArrowBackIcon } from "@mui/icons-material/ArrowBack";
import { default as CenterFocusStrongIcon } from "@mui/icons-material/CenterFocusStrong";
import { default as FolderOpenIcon } from "@mui/icons-material/FolderOpen";
import { default as GridOnIcon } from "@mui/icons-material/GridOn";
import { default as HexagonIcon } from "@mui/icons-material/Hexagon";
import { default as ThreeDRotationIcon } from "@mui/icons-material/ThreeDRotation";
import { default as TuneIcon } from "@mui/icons-material/Tune";
import { AppBar, Box, Divider, IconButton, Toolbar, Tooltip, Typography } from "@mui/material";
import { ReactElement, useCallback } from "react";
import { NavigateFunction, useNavigate } from "react-router-dom";

import { IVisualPreviewViewOptions } from "@/lib/visuals";

interface IVisualPreviewToolbarProps {
  options: IVisualPreviewViewOptions;
  onChangeOptions: (options: IVisualPreviewViewOptions) => void;
  onResetCamera: () => void;
}

/**
 * Preview toolbar.
 *
 * View toggles are live and drive the scene. Everything sourcing data - opening a visual, picking a
 * detail level - is present but disabled, because it needs the rust side that does not exist yet. The
 * placeholders are here so the layout is settled before real commands land in it.
 */
export function VisualPreviewToolbar({
  options,
  onChangeOptions,
  onResetCamera,
}: IVisualPreviewToolbarProps): ReactElement {
  const navigate: NavigateFunction = useNavigate();

  const onBack = useCallback(() => navigate("/visuals_editor", { replace: true }), [navigate]);

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
    <AppBar position={"relative"}>
      <Toolbar variant={"dense"}>
        <IconButton edge={"start"} color={"inherit"} sx={{ marginRight: 1 }} onClick={onBack}>
          <ArrowBackIcon />
        </IconButton>

        <Typography variant={"h6"} component={"div"} noWrap>
          Stub visual
        </Typography>

        <Box sx={{ flexGrow: 1 }} />

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

        <Divider orientation={"vertical"} flexItem sx={{ marginX: 1, borderColor: "rgba(255, 255, 255, 0.3)" }} />

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
      </Toolbar>
    </AppBar>
  );
}

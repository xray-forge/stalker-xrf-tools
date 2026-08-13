import { Box, CircularProgress, LinearProgress, Theme, Typography } from "@mui/material";
import { SystemStyleObject } from "@mui/system";
import { clamp } from "@mui/x-data-grid/internals";
import { useInjection } from "@wirestate/react";
import { MouseEvent, ReactElement, useCallback, useMemo, useState, WheelEvent } from "react";

import { EquipmentGridControls } from "@/applications/equipment-icons/components/sprite-view/EquipmentGridControls";
import { EquipmentGridDetails } from "@/applications/equipment-icons/components/sprite-view/EquipmentGridDetails";
import { EquipmentGridMoveOver } from "@/applications/equipment-icons/components/sprite-view/EquipmentGridMoveOver";
import { EquipmentGridZoom } from "@/applications/equipment-icons/components/sprite-view/EquipmentGridZoom";
import { EquipmentSpriteGrid } from "@/applications/equipment-icons/components/sprite-view/EquipmentSpriteGrid";
import { equipmentViewerConfig } from "@/applications/equipment-icons/configs/EquipmentViewerConfig";
import { IMAGE_CHECKERBOARD } from "@/core/components/media/media.styles";
import { Nullable } from "@/lib/types/general";
import { EquipmentService, GridMapper } from "@/lib/xrf/icons";

export function EquipmentSpriteViewer(): ReactElement {
  const equipmentService: EquipmentService = useInjection(EquipmentService);

  const [holdingOrigin, setHoldingOrigin] = useState<Nullable<[number, number]>>(null);
  const [zoomValue, setZoomValue] = useState(1);
  const [zoomOriginX, setZoomOriginX] = useState(0);
  const [zoomOriginY, setZoomOriginY] = useState(0);

  const [selectedCell, setSelectedCell] = useState<Nullable<[number, number]>>(null);
  const [moveOverCell, setMoveOverCell] = useState<Nullable<[number, number]>>(null);

  const gridMapper: Nullable<GridMapper> = useMemo(() => {
    if (!equipmentService.spriteImage.value) {
      return null;
    }

    return new GridMapper(
      equipmentService.spriteImage.value.image.width,
      equipmentService.spriteImage.value.image.height,
      equipmentService.gridSize,
      equipmentService.spriteImage.value.descriptors
    );
  }, [equipmentService.spriteImage.value, equipmentService.gridSize]);

  const sx: SystemStyleObject<Theme> = useMemo(
    () => ({
      ...IMAGE_CHECKERBOARD,
      backgroundColor: "#353535",
      userSelect: "none",
      transform: `scale(${zoomValue}) translate(${zoomOriginX}px, ${zoomOriginY}px)`,
    }),
    [zoomValue, zoomOriginX, zoomOriginY]
  );

  const onSelectCell = useCallback((row: number, column: number) => {
    setSelectedCell([row, column]);
  }, []);

  const onCloseDetails = useCallback(() => {
    setSelectedCell(null);
  }, []);

  const onMoveOverCell = useCallback((row: number, column: number) => {
    setMoveOverCell((it) => {
      return it && it[0] === row && it[1] === column ? it : [row, column];
    });
  }, []);

  const onZoomUp = useCallback(() => {
    setZoomValue((it) => clamp(it + 0.1, equipmentViewerConfig.ZOOM_IN_MIN, equipmentViewerConfig.ZOOM_IN_MAX));
  }, []);

  const onZoomDown = useCallback(() => {
    setZoomValue((it) => clamp(it - 0.1, equipmentViewerConfig.ZOOM_IN_MIN, equipmentViewerConfig.ZOOM_IN_MAX));
  }, []);

  const onWheel = useCallback((event: WheelEvent<HTMLDivElement>) => {
    if (event.shiftKey) {
      setZoomOriginY((it) =>
        clamp(
          event.deltaY > 0 ? it - 30 : it + 30,
          equipmentViewerConfig.ZOOM_OFFSET_MIN,
          equipmentViewerConfig.ZOOM_OFFSET_MAX
        )
      );
    } else if (event.ctrlKey) {
      setZoomOriginX((it) =>
        clamp(
          event.deltaY > 0 ? it - 30 : it + 30,
          equipmentViewerConfig.ZOOM_OFFSET_MIN,
          equipmentViewerConfig.ZOOM_OFFSET_MAX
        )
      );
    } else {
      setZoomValue((it) =>
        clamp(
          event.deltaY > 0 ? it - 0.1 : it + 0.1,
          equipmentViewerConfig.ZOOM_IN_MIN,
          equipmentViewerConfig.ZOOM_IN_MAX
        )
      );
    }
  }, []);

  const onMouseDown = useCallback((event: MouseEvent<HTMLDivElement>) => {
    setHoldingOrigin([event.pageX, event.pageY]);
  }, []);

  const onMouseUp = useCallback(() => {
    setHoldingOrigin(null);
  }, []);

  const onMouseLeave = useCallback(() => {
    setHoldingOrigin(null);
  }, []);

  const onContextMenu = useCallback((event: MouseEvent<HTMLDivElement>) => {
    event.preventDefault();
  }, []);

  const onMouseMove = useCallback(
    (event: MouseEvent<HTMLDivElement>) => {
      if (holdingOrigin) {
        const [x, y] = holdingOrigin;

        setZoomOriginX((it) =>
          clamp(
            it + (event.pageX - x) / 2,
            equipmentViewerConfig.ZOOM_OFFSET_MIN,
            equipmentViewerConfig.ZOOM_OFFSET_MAX
          )
        );
        setZoomOriginY((it) =>
          clamp(
            it + (event.pageY - y) / 2,
            equipmentViewerConfig.ZOOM_OFFSET_MIN,
            equipmentViewerConfig.ZOOM_OFFSET_MAX
          )
        );
        setHoldingOrigin([event.pageX, event.pageY]);
      }
    },
    [holdingOrigin]
  );

  return (
    <Box sx={{ width: "100%", height: "100%", position: "relative", overflow: "hidden" }}>
      {equipmentService.spriteImage.isLoading && equipmentService.spriteImage.value ? (
        <LinearProgress sx={{ position: "absolute", left: 0, top: 0, width: "100%", zIndex: 2 }} />
      ) : null}

      <Box
        sx={{
          position: "absolute",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
          left: 0,
          top: 0,
          width: "100%",
          height: "100%",
        }}
      >
        {equipmentService.spriteImage.value ? (
          <Box
            className={"sprite-preview"}
            onWheel={onWheel}
            onMouseDown={onMouseDown}
            onMouseUp={onMouseUp}
            onMouseLeave={onMouseLeave}
            onContextMenu={onContextMenu}
            onMouseMove={onMouseMove}
            sx={[
              {
                position: "relative",
                width: equipmentService.spriteImage.value.image.width,
                minWidth: equipmentService.spriteImage.value.image.width,
                height: "auto",
                left: 0,
                top: 0,
              },
              sx,
            ]}
          >
            <img src={equipmentService.spriteImage.value.image.src} width={"100%"} height={"100%"} draggable={false} />

            {gridMapper ? (
              <EquipmentSpriteGrid
                selectedCell={selectedCell}
                isGridVisible={equipmentService.isGridVisible}
                gridMapper={gridMapper}
                onCellSelected={onSelectCell}
                onCellMovedOver={onMoveOverCell}
              />
            ) : null}
          </Box>
        ) : equipmentService.spriteImage.isLoading ? (
          <CircularProgress size={28} />
        ) : (
          <Typography variant={"body2"} color={"text.secondary"}>
            No sprite open
          </Typography>
        )}

        {selectedCell && gridMapper ? (
          <EquipmentGridDetails cell={selectedCell} gridMapper={gridMapper} onClose={onCloseDetails} />
        ) : null}

        {moveOverCell ? <EquipmentGridMoveOver cell={moveOverCell} /> : null}

        <EquipmentGridControls
          gridSize={equipmentService.gridSize}
          isGridVisible={equipmentService.isGridVisible}
          onSetGridSize={equipmentService.setGridSize}
          onSetGridVisibility={equipmentService.setGridVisibility}
        />

        <EquipmentGridZoom zoom={zoomValue} onZoomDown={onZoomDown} onZoomUp={onZoomUp} />
      </Box>
    </Box>
  );
}

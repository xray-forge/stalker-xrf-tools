import { Grid } from "@mui/material";
import { SxProps } from "@mui/system";
import { clamp } from "@mui/x-data-grid/internals";
import { useManager } from "dreamstate";
import { MouseEvent, ReactElement, useCallback, useMemo, useState, WheelEvent } from "react";

import { EquipmentGridControls } from "@/applications/icons_editor/components/sprite_view/EquipmentGridControls";
import { EquipmentGridDetails } from "@/applications/icons_editor/components/sprite_view/EquipmentGridDetails";
import { EquipmentGridMoveOver } from "@/applications/icons_editor/components/sprite_view/EquipmentGridMoveOver";
import { EquipmentGridZoom } from "@/applications/icons_editor/components/sprite_view/EquipmentGridZoom";
import { EquipmentSpriteGrid } from "@/applications/icons_editor/components/sprite_view/EquipmentSpriteGrid";
import { equipmentViewerConfig } from "@/applications/icons_editor/configs/EquipmentViewerConfig";
import { EquipmentManager } from "@/applications/icons_editor/store/equipment";
import { Optional } from "@/core/types/general";
import { GridMapper } from "@/lib/icons";

export function EquipmentSpriteViewer({
  equipmentContext: { spriteImage: { value: spriteImage }, gridSize, isGridVisible, equipmentActions } = useManager(
    EquipmentManager
  ),
}): ReactElement {
  const [holdingOrigin, setHoldingOrigin] = useState<Optional<[number, number]>>(null);
  const [zoomValue, setZoomValue] = useState(1);
  const [zoomOriginX, setZoomOriginX] = useState(0);
  const [zoomOriginY, setZoomOriginY] = useState(0);

  const [selectedCell, setSelectedCell] = useState<Optional<[number, number]>>(null);
  const [moveOverCell, setMoveOverCell] = useState<Optional<[number, number]>>(null);

  const gridMapper: Optional<GridMapper> = useMemo(
    () =>
      spriteImage
        ? new GridMapper(spriteImage.image.width, spriteImage.image.height, gridSize, spriteImage.descriptors)
        : null,
    [spriteImage, gridSize]
  );

  const sx: SxProps = useMemo(
    () => ({
      backgroundColor: "#353535",
      backgroundImage:
        "linear-gradient(45deg, #707070 25%, transparent 25%), linear-gradient(-45deg, #808080 25%, transparent 25%)," +
        "linear-gradient(45deg, transparent 75%, #808080 75%), linear-gradient(-45deg, transparent 75%, #808080 75%)",
      backgroundSize: "20px 20px",
      backgroundPosition: "0 0, 0 10px, 10px -10px, -10px 0px",
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

  const onWheel = useCallback(
    (event: WheelEvent<HTMLDivElement>) => {
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
    },
    [zoomValue]
  );

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
    <Grid width={"100%"} height={"100%"} position={"relative"} overflow={"hidden"}>
      <Grid
        position={"absolute"}
        display={"flex"}
        justifyContent={"center"}
        alignItems={"center"}
        left={0}
        top={0}
        width={"100%"}
        height={"100%"}
        bgcolor={"#353535"}
      >
        {spriteImage ? (
          <Grid
            position={"relative"}
            className={"sprite-preview"}
            width={spriteImage.image.width}
            minWidth={spriteImage.image.width}
            height={"auto"}
            left={0}
            top={0}
            sx={sx}
            onWheel={onWheel}
            onMouseDown={onMouseDown}
            onMouseUp={onMouseUp}
            onMouseLeave={onMouseLeave}
            onContextMenu={onContextMenu}
            onMouseMove={onMouseMove}
          >
            <img src={spriteImage.image.src} width={"100%"} height={"100%"} draggable={false} />

            {gridMapper ? (
              <EquipmentSpriteGrid
                selectedCell={selectedCell}
                isGridVisible={isGridVisible}
                gridMapper={gridMapper}
                onCellSelected={onSelectCell}
                onCellMovedOver={onMoveOverCell}
              />
            ) : null}
          </Grid>
        ) : (
          "loading..."
        )}

        {selectedCell && gridMapper ? (
          <EquipmentGridDetails cell={selectedCell} gridMapper={gridMapper} onClose={onCloseDetails} />
        ) : null}

        {moveOverCell ? <EquipmentGridMoveOver cell={moveOverCell} /> : null}

        <EquipmentGridControls
          gridSize={gridSize}
          isGridVisible={isGridVisible}
          onSetGridSize={equipmentActions.setGridSize}
          onSetGridVisibility={equipmentActions.setGridVisibility}
        />

        <EquipmentGridZoom zoom={zoomValue} onZoomDown={onZoomDown} onZoomUp={onZoomUp} />
      </Grid>
    </Grid>
  );
}

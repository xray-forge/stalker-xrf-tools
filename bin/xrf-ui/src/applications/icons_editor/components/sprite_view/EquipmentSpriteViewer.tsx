import { Grid } from "@mui/material";
import { SxProps } from "@mui/system";
import { clamp } from "@mui/x-data-grid/internals";
import { useManager } from "dreamstate";
import { MouseEvent, ReactElement, useCallback, useMemo, useState, WheelEvent } from "react";

import { EquipmentGridControls } from "@/applications/icons_editor/components/sprite_view/EquipmentGridControls";
import { EquipmentGridZoom } from "@/applications/icons_editor/components/sprite_view/EquipmentGridZoom";
import { EquipmentSpriteGrid } from "@/applications/icons_editor/components/sprite_view/EquipmentSpriteGrid";
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

  const onZoomUp = useCallback(() => {
    setZoomValue((it) => clamp(it + 0.1, 0.1, 5));
  }, []);

  const onZoomDown = useCallback(() => {
    setZoomValue((it) => clamp(it - 0.1, 0.1, 5));
  }, []);

  const onWheel = useCallback(
    (event: WheelEvent<HTMLDivElement>) => {
      if (event.shiftKey) {
        setZoomOriginY((it) => clamp(event.deltaY > 0 ? it - 30 : it + 30, -2000, 2000));
      } else if (event.ctrlKey) {
        setZoomOriginX((it) => clamp(event.deltaY > 0 ? it - 30 : it + 30, -2000, 2000));
      } else {
        setZoomValue((it) => clamp(event.deltaY > 0 ? it - 0.1 : it + 0.1, 0.1, 5));
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

  const onMouseMove = useCallback(
    (event: MouseEvent<HTMLDivElement>) => {
      if (holdingOrigin) {
        const [x, y] = holdingOrigin;

        setZoomOriginX((it) => clamp(it + (event.pageX - x) / 2, -2000, 2000));
        setZoomOriginY((it) => clamp(it + (event.pageY - y) / 2, -2000, 2000));
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
        onWheel={onWheel}
        onMouseDown={onMouseDown}
        onMouseUp={onMouseUp}
        onMouseMove={onMouseMove}
        onMouseLeave={onMouseLeave}
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
          >
            <img src={spriteImage.image.src} width={"100%"} height={"100%"} draggable={false} />

            {gridMapper ? <EquipmentSpriteGrid isGridVisible={isGridVisible} gridMapper={gridMapper} /> : null}
          </Grid>
        ) : (
          "loading..."
        )}

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

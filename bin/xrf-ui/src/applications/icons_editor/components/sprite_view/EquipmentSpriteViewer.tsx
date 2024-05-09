import { default as AddIcon } from "@mui/icons-material/AddCircle";
import { default as RemoveIcon } from "@mui/icons-material/RemoveCircle";
import { Button, Grid, IconButton } from "@mui/material";
import { SxProps } from "@mui/system";
import { clamp } from "@mui/x-data-grid/internals";
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { MouseEvent, ReactElement, useCallback, useEffect, useMemo, useRef, useState, WheelEvent } from "react";

import { EquipmentSpriteGrid } from "@/applications/icons_editor/components/sprite_view/EquipmentSpriteGrid";
import { Optional } from "@/core/types/general";
import { EIconsEditorCommand } from "@/lib/ipc";

export function EquipmentSpriteViewer(): ReactElement {
  const [sourceUrl, setSourceUrl] = useState<Optional<string>>(null);

  const [holdingOrigin, setHoldingOrigin] = useState<Optional<[number, number]>>(null);
  const [zoomValue, setZoomValue] = useState(1);
  const [zoomOriginX, setZoomOriginX] = useState(0);
  const [zoomOriginY, setZoomOriginY] = useState(0);

  const imageWrapperRef = useRef<Optional<HTMLDivElement>>(null);

  const sx: SxProps = useMemo(
    () => ({
      backgroundColor: "#353535",
      backgroundImage:
        "linear-gradient(45deg, #707070 25%, transparent 25%), linear-gradient(-45deg, #808080 25%, transparent 25%)," +
        "linear-gradient(45deg, transparent 75%, #808080 75%), linear-gradient(-45deg, transparent 75%, #808080 75%)",
      backgroundSize: "20px 20px",
      backgroundPosition: "0 0, 0 10px, 10px -10px, -10px 0px",
      userSelect: "none",
      transform: `scale(${zoomValue})`,
    }),
    [zoomValue, zoomOriginX, zoomOriginY]
  );

  const onResetClick = useCallback(() => {
    setZoomOriginX(0);
    setZoomOriginY(0);
    setZoomValue(1);
  }, []);

  const onZoomUp = useCallback(() => {
    setZoomValue((it) => clamp(it + 0.1, 0.1, 1.5));
  }, []);

  const onZoomDown = useCallback(() => {
    setZoomValue((it) => clamp(it - 0.1, 0.1, 1.5));
  }, []);

  const onWheel = useCallback(
    (event: WheelEvent<HTMLDivElement>) => {
      if (event.shiftKey) {
        setZoomOriginY((it) => clamp(event.deltaY > 0 ? it - 30 : it + 30, -2000, 2000));
      } else if (event.ctrlKey) {
        setZoomOriginX((it) => clamp(event.deltaY > 0 ? it - 30 : it + 30, -2000, 2000));
      } else {
        setZoomValue((it) => clamp(event.deltaY > 0 ? it - 0.1 : it + 0.1, 0.1, 1.5));
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

        setZoomOriginX((it) => it + (event.pageX - x));
        setZoomOriginY((it) => it + (event.pageY - y));
        setHoldingOrigin([event.pageX, event.pageY]);
      }
    },
    [holdingOrigin]
  );

  useEffect(() => {
    invoke(EIconsEditorCommand.GET_EQUIPMENT_SPRITE_URI)
      .then((uri) => {
        setSourceUrl(uri as Optional<string>);
      })
      .catch(console.error);
  }, []);

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
        {sourceUrl ? (
          <Grid
            ref={imageWrapperRef}
            position={"relative"}
            className={"sprite-preview"}
            height={"auto"}
            left={zoomOriginX}
            top={zoomOriginY}
            sx={sx}
          >
            <img src={convertFileSrc(sourceUrl, "stream")} width={"100%"} height={"100%"} draggable={false} />

            <EquipmentSpriteGrid isGridVisible={true} gridSize={50} rowsCount={38} columnsCount={20} />
          </Grid>
        ) : (
          "loading..."
        )}

        <Grid display={"flex"} alignItems={"center"} position={"absolute"} right={4} bottom={4}>
          <IconButton aria-label={"delete"} size={"small"} color={"primary"} onClick={onZoomUp}>
            <AddIcon />
          </IconButton>

          <Grid marginLeft={0.5} marginRight={0.5}>
            {zoomValue.toFixed(2)}
          </Grid>

          <IconButton aria-label={"delete"} size={"small"} color={"primary"} onClick={onZoomDown}>
            <RemoveIcon />
          </IconButton>

          <Button size={"small"} onClick={onResetClick}>
            RESET
          </Button>
        </Grid>
      </Grid>
    </Grid>
  );
}

import { Grid } from "@mui/material";
import { SxProps } from "@mui/system";
import { memo, ReactElement, useMemo } from "react";

interface IEquipmentSpriteGridProps {
  isGridVisible: boolean;
  zoomCoefficient?: number;
  gridSize?: number;
  rowsCount: number;
  columnsCount: number;
}

export const EquipmentSpriteGrid = memo(
  ({
    isGridVisible,
    zoomCoefficient = 1,
    gridSize = 50,
    rowsCount,
    columnsCount,
  }: IEquipmentSpriteGridProps): ReactElement => {
    const size: number = zoomCoefficient * gridSize;

    const sx: SxProps = useMemo(
      () => ({
        userSelect: "none",
        border: isGridVisible ? "1px solid" : "none",
        borderColor: "rgba(0,2,1,0.54)",
        "&:hover": {
          background: "rgba(0,0,0,0.49)",
        },
        "&:hover .coordinates": {
          display: "flex",
          userSelect: "none",
        },
      }),
      [isGridVisible]
    );

    return (
      <Grid position={"absolute"} left={0} top={0} right={0} bottom={0} bgcolor={"#66666608"}>
        {Array(rowsCount)
          .fill(1)
          .map((_, row) => (
            <Grid display={"flex"} key={row}>
              {Array(columnsCount)
                .fill(1)
                .map((_, column) => (
                  <Grid
                    key={column}
                    display={"flex"}
                    flexWrap={"nowrap"}
                    justifyContent={"center"}
                    alignItems={"center"}
                    minHeight={size}
                    maxHeight={size}
                    minWidth={size}
                    maxWidth={size}
                    sx={sx}
                  >
                    <Grid className={"coordinates"} display={"none"}>
                      {row}:{column}
                    </Grid>
                  </Grid>
                ))}
            </Grid>
          ))}
      </Grid>
    );
  }
);

EquipmentSpriteGrid.displayName = "EquipmentSpriteGrid";

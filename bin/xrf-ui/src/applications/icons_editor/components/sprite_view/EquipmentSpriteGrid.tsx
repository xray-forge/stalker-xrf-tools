import { Grid } from "@mui/material";
import { memo, ReactElement } from "react";

interface IEquipmentSpriteGridProps {
  isGridVisible: boolean;
  zoomCoefficient?: number;
  gridSize?: number;
  rowsCount: number;
  columnsCount: number;
}

export const EquipmentSpriteGrid = memo(
  ({ zoomCoefficient = 1, gridSize = 50, rowsCount, columnsCount }: IEquipmentSpriteGridProps): ReactElement => {
    const size: number = zoomCoefficient * gridSize;

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
                    sx={{
                      userSelect: "none",
                      border: "1px solid",
                      borderColor: "rgba(0,251,105,0.25)",
                      "&:hover": {
                        background: "rgba(0,251,105,0.25)",
                      },
                      "&:hover .coordinates": {
                        display: "flex",
                      },
                    }}
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

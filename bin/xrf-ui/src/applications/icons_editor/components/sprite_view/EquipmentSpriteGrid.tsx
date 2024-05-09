import { Grid } from "@mui/material";
import { SxProps } from "@mui/system";
import { memo, ReactElement, useMemo } from "react";

import { GridMapper } from "@/lib/icons";

interface IEquipmentSpriteGridProps {
  isGridVisible: boolean;
  gridMapper: GridMapper;
}

export const EquipmentSpriteGrid = memo(({ isGridVisible, gridMapper }: IEquipmentSpriteGridProps): ReactElement => {
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
      {gridMapper.grid.map((row, rowIndex) => (
        <Grid display={"flex"} key={rowIndex}>
          {row.map((column, columnIndex) => (
            <Grid
              key={columnIndex}
              display={"flex"}
              flexWrap={"nowrap"}
              justifyContent={"center"}
              alignItems={"center"}
              minHeight={gridMapper.gridSize}
              maxHeight={gridMapper.gridSize}
              minWidth={gridMapper.gridSize}
              maxWidth={gridMapper.gridSize}
              sx={sx}
            >
              <Grid className={"coordinates"} display={"none"}>
                {rowIndex}:{columnIndex} ({column?.length ?? 0})
              </Grid>
            </Grid>
          ))}
        </Grid>
      ))}
    </Grid>
  );
});

EquipmentSpriteGrid.displayName = "EquipmentSpriteGrid";

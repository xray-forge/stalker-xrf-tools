import { Grid } from "@mui/material";
import { SxProps } from "@mui/system";
import { memo, ReactElement, useMemo } from "react";

import { Optional } from "@/core/types/general";
import { GridMapper, TEquipmentCell } from "@/lib/icons";

interface IEquipmentSpriteGridProps {
  isGridVisible: boolean;
  selectedCell: Optional<TEquipmentCell>;
  gridMapper: GridMapper;
  onCellSelected: (row: number, column: number) => void;
  onCellMovedOver: (row: number, column: number) => void;
}

export const EquipmentSpriteGrid = memo(
  ({
    selectedCell,
    isGridVisible,
    gridMapper,
    onCellSelected,
    onCellMovedOver,
  }: IEquipmentSpriteGridProps): ReactElement => {
    const sx: SxProps = useMemo(
      () => ({
        userSelect: "none",
        border: isGridVisible ? "1px solid" : "none",
        borderColor: "rgba(2,2,2,0.6)",
        "&.selected": {
          background: "rgba(39,48,117,0.49)",
        },
        "&.selected:hover": {
          background: "rgba(33,77,172,0.49)",
        },
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
                className={
                  selectedCell && selectedCell[0] === rowIndex && selectedCell[1] === columnIndex ? "selected" : ""
                }
                display={"flex"}
                flexWrap={"nowrap"}
                justifyContent={"center"}
                alignItems={"center"}
                minHeight={gridMapper.gridSize}
                maxHeight={gridMapper.gridSize}
                minWidth={gridMapper.gridSize}
                maxWidth={gridMapper.gridSize}
                sx={sx}
                onClick={() => onCellSelected(rowIndex, columnIndex)}
                onMouseMove={() => onCellMovedOver(rowIndex, columnIndex)}
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
  }
);

EquipmentSpriteGrid.displayName = "EquipmentSpriteGrid";

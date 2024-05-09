import { default as AddIcon } from "@mui/icons-material/AddCircle";
import { default as RemoveIcon } from "@mui/icons-material/RemoveCircle";
import { FormControlLabel, Grid, IconButton, Switch } from "@mui/material";
import { ReactElement, useCallback } from "react";

interface IEquipmentGridControlsProps {
  isGridVisible: boolean;
  gridSize: number;
  onSetGridSize: (size: number) => void;
  onSetGridVisibility: (isVisible: boolean) => void;
}

export function EquipmentGridControls({
  isGridVisible,
  gridSize,
  onSetGridSize,
  onSetGridVisibility,
}: IEquipmentGridControlsProps): ReactElement {
  const onGridVisibilityToggled = useCallback(() => {
    onSetGridVisibility(!isGridVisible);
  }, [isGridVisible]);

  const onDecreaseGridSize = useCallback(() => {
    onSetGridSize(gridSize - 5);
  }, [gridSize]);

  const onIncreaseGridSize = useCallback(() => {
    onSetGridSize(gridSize + 5);
  }, [gridSize]);

  return (
    <Grid display={"flex"} alignItems={"center"} position={"absolute"} right={4} top={4}>
      <FormControlLabel
        label={"Grid"}
        labelPlacement={"start"}
        control={<Switch size={"small"} checked={isGridVisible} onChange={onGridVisibilityToggled} />}
      />

      <Grid margin={1} />

      <IconButton size={"small"} onClick={onDecreaseGridSize}>
        <RemoveIcon />
      </IconButton>

      <Grid marginLeft={0.5} marginRight={0.5}>
        {gridSize}
      </Grid>

      <IconButton size={"small"} onClick={onIncreaseGridSize}>
        <AddIcon />
      </IconButton>
    </Grid>
  );
}

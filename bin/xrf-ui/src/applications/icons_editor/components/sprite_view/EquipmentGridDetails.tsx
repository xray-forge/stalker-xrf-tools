import { default as CloseIcon } from "@mui/icons-material/Close";
import { Card, Divider, Grid, IconButton, Typography } from "@mui/material";
import { ReactElement } from "react";

import { Optional } from "@/core/types/general";
import { stopPropagation } from "@/lib/event";
import { GridMapper, IEquipmentSectionDescriptor, TEquipmentCell } from "@/lib/icons";

interface IEquipmentGridDetailsProps {
  cell: TEquipmentCell;
  gridMapper: GridMapper;
  onClose: () => void;
}

export function EquipmentGridDetails({ gridMapper, cell, onClose }: IEquipmentGridDetailsProps): ReactElement {
  const [row, column] = cell;
  const items: Optional<Array<IEquipmentSectionDescriptor>> = gridMapper.grid[row][column] ?? null;

  const list = items?.map((it, index) => (
    <Grid key={index} marginTop={"4px"}>
      {it.section}
    </Grid>
  ));

  return (
    <Grid
      position={"absolute"}
      left={4}
      top={4}
      maxWidth={300}
      minWidth={160}
      maxHeight={"50%"}
      overflow={"auto"}
      onMouseMove={stopPropagation}
      onWheel={stopPropagation}
      onMouseDown={stopPropagation}
    >
      <Card>
        <Grid direction={"column"} padding={1} margin={0} width={"100%"} spacing={0.5} container={true}>
          <Grid justifyContent={"space-between"} alignItems={"center"} marginBottom={1} container>
            <Typography variant={"h6"}>{`${cell[1]}:${cell[0]}`}</Typography>

            <IconButton size={"small"} onClick={onClose}>
              <CloseIcon />
            </IconButton>
          </Grid>

          <Divider />

          {list?.length ? list : "No sprites"}
        </Grid>
      </Card>
    </Grid>
  );
}

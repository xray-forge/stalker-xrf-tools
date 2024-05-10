import { Grid } from "@mui/material";
import { ReactElement } from "react";

import { TEquipmentCell } from "@/lib/icons";

interface IEquipmentGridMoveOver {
  cell: TEquipmentCell;
}

export function EquipmentGridMoveOver({ cell }: IEquipmentGridMoveOver): ReactElement {
  return (
    <Grid position={"absolute"} left={4} bottom={4}>
      {`${cell[0]}:${cell[1]}`}
    </Grid>
  );
}

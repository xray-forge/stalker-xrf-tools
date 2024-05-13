import { Chip, Grid } from "@mui/material";
import { ReactElement } from "react";

import { IPackEquipmentResult } from "@/lib/icons";

interface IEquipmentPackResultProps {
  result: IPackEquipmentResult;
}

export function EquipmentPackResult({ result }: IEquipmentPackResultProps): ReactElement {
  return (
    <Grid direction={"column"} padding={2} maxWidth={540} container>
      <Grid justifyContent={"center"} gap={1} container>
        <Chip variant={"outlined"} color={"success"} label={`${result.duration / 1000} sec`} />
        <Chip
          variant={"outlined"}
          color={"success"}
          label={`${result.packedCount + result.skippedCount} files total`}
        />
      </Grid>

      <Grid justifyContent={"center"} gap={1} marginTop={1} padding={`0 ${16}px`} container>
        <Chip variant={"outlined"} label={`${result.packedCount} file(s) packed`} />
        <Chip variant={"outlined"} label={`${result.skippedCount} file(s) skipped`} />
        <Chip variant={"outlined"} label={`${result.savedWidth}x${result.savedHeight} sprite`} />
      </Grid>
    </Grid>
  );
}

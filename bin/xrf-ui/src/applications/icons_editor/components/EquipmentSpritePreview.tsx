import { Grid } from "@mui/material";
import { SxProps } from "@mui/system";
import { ReactElement, useMemo } from "react";

interface IEquipmentSpriteViewProps {
  src: string;
  width: number | string;
  height: number | string;
}

export function EquipmentSpritePreview({ src, width, height }: IEquipmentSpriteViewProps): ReactElement {
  const sx: SxProps = useMemo(
    () => ({
      backgroundColor: "#393535",
      backgroundImage:
        "linear-gradient(45deg, #707070 25%, transparent 25%), linear-gradient(-45deg, #808080 25%, transparent 25%)," +
        "linear-gradient(45deg, transparent 75%, #808080 75%), linear-gradient(-45deg, transparent 75%, #808080 75%)",
      backgroundSize: "40px 40px",
      backgroundPosition: "0 0, 0 20px, 20px -20px, -20px 0px",
    }),
    []
  );

  return (
    <Grid className={"sprite-preview"} width={width} height={height} sx={sx}>
      <img src={src} width={"100%"} height={"100%"} />
    </Grid>
  );
}

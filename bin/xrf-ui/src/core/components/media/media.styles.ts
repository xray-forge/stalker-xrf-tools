import { Theme } from "@mui/material";
import { SystemStyleObject } from "@mui/system";

/**
 * Alpha checkerboard, so a transparent region reads as transparent rather than as black.
 */
export const IMAGE_CHECKERBOARD: SystemStyleObject<Theme> = {
  backgroundImage: [
    "linear-gradient(45deg, #707070 25%, transparent 25%)",
    "linear-gradient(-45deg, #808080 25%, transparent 25%)",
    "linear-gradient(45deg, transparent 75%, #808080 75%)",
    "linear-gradient(-45deg, transparent 75%, #808080 75%)",
  ].join(","),
  backgroundSize: "20px 20px",
  backgroundPosition: "0 0, 0 10px, 10px -10px, -10px 0px",
};

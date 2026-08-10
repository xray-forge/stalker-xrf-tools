import { Box, Typography } from "@mui/material";
import { ReactElement } from "react";

const HEADER_FIELDS: Array<string> = ["Format version", "Model type", "Shader", "Texture", "Bounding box", "Source"];

function renderEmpty(label: string): ReactElement {
  return (
    <Typography variant={"body2"} sx={{ opacity: 0.6 }}>
      {label}
    </Typography>
  );
}

/**
 * The panels behind the visuals tool stripe.
 */
export function VisualHeaderPanel(): ReactElement {
  return (
    <Box sx={{ padding: 2 }}>
      {HEADER_FIELDS.map((field) => (
        <Box key={field} sx={{ display: "flex", justifyContent: "space-between", paddingY: 0.5 }}>
          <Typography variant={"body2"} sx={{ opacity: 0.6 }}>
            {field}
          </Typography>
          <Typography variant={"body2"}>&mdash;</Typography>
        </Box>
      ))}
    </Box>
  );
}

// todo: Split.
export function VisualBonesPanel(): ReactElement {
  return <Box sx={{ padding: 2 }}>{renderEmpty("No skeleton. Ogf bone and ik chunks land here.")}</Box>;
}

// todo: Split.
export function VisualMotionsPanel(): ReactElement {
  return <Box sx={{ padding: 2 }}>{renderEmpty("No motions. Resolved from the visual's omf motion refs.")}</Box>;
}

// todo: Split.
export function VisualMaterialsPanel(): ReactElement {
  return <Box sx={{ padding: 2 }}>{renderEmpty("No materials. Texture and shader names per child visual.")}</Box>;
}

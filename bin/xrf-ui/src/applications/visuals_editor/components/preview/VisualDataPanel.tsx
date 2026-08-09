import { Box, Divider, Drawer, Tab, Tabs, Typography } from "@mui/material";
import { ReactElement, useMemo } from "react";

import { useTabState } from "@/lib/tab";

const HEADER_FIELDS: Array<string> = ["Format version", "Model type", "Shader", "Texture", "Bounding box", "Source"];

function renderEmpty(label: string): ReactElement {
  return (
    <Typography variant={"body2"} sx={{ opacity: 0.6 }}>
      {label}
    </Typography>
  );
}

/**
 * Right side panel for the data behind the rendered visual.
 *
 * Placeholder content only. The tabs mirror what an ogf and its omf motions actually carry, so the
 * panel keeps its shape once the rust side starts feeding it: header and material info from the ogf
 * chunks, the bone tree, and the motion list resolved through the visual's motion refs.
 */
export function VisualDataPanel(): ReactElement {
  const [activeTab, , onActiveTabChange] = useTabState<string>("header");

  const activeContent: ReactElement = useMemo(() => {
    switch (activeTab) {
      case "header":
        return (
          <Box>
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

      case "bones":
        return renderEmpty("No skeleton. Ogf bone and ik chunks land here.");

      case "motions":
        return renderEmpty("No motions. Resolved from the visual's omf motion refs.");

      case "materials":
        return renderEmpty("No materials. Texture and shader names per child visual.");

      default:
        return renderEmpty("Unknown tab");
    }
  }, [activeTab]);

  return (
    <Drawer
      anchor={"right"}
      variant={"permanent"}
      open={true}
      sx={{ height: "100%", width: 300, flexShrink: 0 }}
      slotProps={{ paper: { sx: { position: "relative", width: 300 } } }}
    >
      <Tabs value={activeTab} variant={"scrollable"} scrollButtons={"auto"} onChange={onActiveTabChange}>
        <Tab value={"header"} label={"Header"} />
        <Tab value={"bones"} label={"Bones"} />
        <Tab value={"motions"} label={"Motions"} />
        <Tab value={"materials"} label={"Materials"} />
      </Tabs>

      <Divider />

      <Box sx={{ padding: 2, flexGrow: 1, minHeight: 0, overflowY: "auto" }}>{activeContent}</Box>
    </Drawer>
  );
}

import { default as AccountTreeIcon } from "@mui/icons-material/AccountTree";
import { default as AnimationIcon } from "@mui/icons-material/Animation";
import { default as InfoIcon } from "@mui/icons-material/Info";
import { default as LayersIcon } from "@mui/icons-material/Layers";

import { VisualBonesPanel } from "@/applications/visuals-explorer/components/panels/VisualBonesPanel";
import { VisualHeaderPanel } from "@/applications/visuals-explorer/components/panels/VisualHeaderPanel";
import { VisualMaterialsPanel } from "@/applications/visuals-explorer/components/panels/VisualMaterialsPanel";
import { VisualMotionsPanel } from "@/applications/visuals-explorer/components/panels/VisualMotionsPanel";
import { IEditorPanel } from "@/core/shell/panel/context";

/**
 * What the viewer contributes to the right panel stripe.
 */
export const VISUAL_EXPLORER_PANELS: Array<IEditorPanel> = [
  { id: "header", label: "Header", icon: <InfoIcon />, render: () => <VisualHeaderPanel /> },
  { id: "bones", label: "Bones", icon: <AccountTreeIcon />, render: () => <VisualBonesPanel /> },
  { id: "motions", label: "Motions", icon: <AnimationIcon />, render: () => <VisualMotionsPanel /> },
  { id: "materials", label: "Materials", icon: <LayersIcon />, render: () => <VisualMaterialsPanel /> },
];

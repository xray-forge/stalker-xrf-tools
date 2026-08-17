import { default as AccountTreeIcon } from "@mui/icons-material/AccountTree";
import { default as AnimationIcon } from "@mui/icons-material/Animation";
import { default as InfoIcon } from "@mui/icons-material/Info";
import { default as LayersIcon } from "@mui/icons-material/Layers";

import { VisualBonesPanel } from "@/applications/visuals-viewer/components/panels/VisualBonesPanel";
import { VisualHeaderPanel } from "@/applications/visuals-viewer/components/panels/VisualHeaderPanel";
import { VisualMaterialsPanel } from "@/applications/visuals-viewer/components/panels/VisualMaterialsPanel";
import { VisualMotionsPanel } from "@/applications/visuals-viewer/components/panels/VisualMotionsPanel";
import { IEditorPanel } from "@/core/shell/panel/context";

/**
 * What the viewer contributes to the right panel stripe.
 */
export const VISUAL_VIEWER_PANELS: Array<IEditorPanel> = [
  { id: "header", label: "Header", icon: <InfoIcon />, render: () => <VisualHeaderPanel /> },
  { id: "bones", label: "Bones", icon: <AccountTreeIcon />, render: () => <VisualBonesPanel /> },
  { id: "motions", label: "Motions", icon: <AnimationIcon />, render: () => <VisualMotionsPanel /> },
  { id: "materials", label: "Materials", icon: <LayersIcon />, render: () => <VisualMaterialsPanel /> },
];

import { default as AccountTreeIcon } from "@mui/icons-material/AccountTree";
import { default as AnimationIcon } from "@mui/icons-material/Animation";
import { default as InfoIcon } from "@mui/icons-material/Info";
import { default as LayersIcon } from "@mui/icons-material/Layers";

import {
  VisualBonesPanel,
  VisualHeaderPanel,
  VisualMaterialsPanel,
  VisualMotionsPanel,
} from "@/applications/visuals-editor/components/preview/VisualDataPanel";
import { IEditorTool } from "@/core/components/shell/EditorToolsContext";

/**
 * What the visuals editor contributes to the tool stripe.
 *
 * Declared as data so the shell decides placement, and so the set is comparable by id without the
 * editor having to memoise it.
 */
export const VISUAL_EDITOR_TOOLS: Array<IEditorTool> = [
  { id: "header", label: "Header", icon: <InfoIcon />, render: () => <VisualHeaderPanel /> },
  { id: "bones", label: "Bones", icon: <AccountTreeIcon />, render: () => <VisualBonesPanel /> },
  { id: "motions", label: "Motions", icon: <AnimationIcon />, render: () => <VisualMotionsPanel /> },
  { id: "materials", label: "Materials", icon: <LayersIcon />, render: () => <VisualMaterialsPanel /> },
];

import { IEditorTool } from "@/core/components/shell/EditorToolsContext";
import { NotificationsPanel } from "@/core/components/shell/notifications/NotificationsPanel";
import { NotificationsToolIcon } from "@/core/components/shell/notifications/NotificationsToolIcon";

export const NOTIFICATIONS_TOOL_ID: string = "notifications";

/**
 * Panels the frame owns rather than the active editor.
 */
export const GLOBAL_TOOLS: Array<IEditorTool> = [
  {
    icon: <NotificationsToolIcon />,
    id: NOTIFICATIONS_TOOL_ID,
    isOpenByDefault: false,
    label: "Notifications",
    render: () => <NotificationsPanel />,
  },
];

export function isGlobalToolId(id: string): boolean {
  return GLOBAL_TOOLS.some((tool: IEditorTool) => tool.id === id);
}

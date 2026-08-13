import { ReactNode } from "react";

/** Which stripe owns a panel. */
export type TEditorPanelSide = "left" | "right";

export interface IEditorPanel {
  id: string;
  label: string;
  icon: ReactNode;
  /** Left is where an application browses, right is where it inspects. */
  side?: TEditorPanelSide;
  isOpenByDefault?: boolean;
  render: () => ReactNode;
}

/** Panels on one side, in declaration order. */
export function selectPanelsOnSide(panels: ReadonlyArray<IEditorPanel>, side: TEditorPanelSide): Array<IEditorPanel> {
  return panels.filter((panel: IEditorPanel) => (panel.side ?? "right") === side);
}

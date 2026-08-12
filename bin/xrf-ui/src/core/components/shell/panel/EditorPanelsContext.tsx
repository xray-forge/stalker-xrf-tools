import { createContext, ReactElement, ReactNode, useContext, useEffect, useMemo, useRef, useState } from "react";

/** Which stripe owns a panel. */
export type TEditorPanelSide = "left" | "right";

export interface IEditorPanel {
  id: string;
  label: string;
  icon: ReactNode;
  /**
   * Left is where an application browses, right is where it inspects.
   */
  side?: TEditorPanelSide;
  isOpenByDefault?: boolean;
  render: () => ReactNode;
}

interface IEditorPanelsContextValue {
  panels: Array<IEditorPanel>;
  setPanels: (panels: Array<IEditorPanel>) => void;
}

const EditorPanelsContext = createContext<IEditorPanelsContextValue>({
  panels: [],
  setPanels: () => {},
});

export function EditorPanelsProvider({ children }: { children: ReactNode }): ReactElement {
  const [panels, setPanels] = useState<Array<IEditorPanel>>([]);

  const value: IEditorPanelsContextValue = useMemo(() => ({ panels, setPanels }), [panels]);

  return <EditorPanelsContext.Provider value={value}>{children}</EditorPanelsContext.Provider>;
}

export function useEditorPanelsRegistry(): Array<IEditorPanel> {
  return useContext(EditorPanelsContext).panels;
}

/** Panels on one side, in declaration order. */
export function selectPanelsOnSide(panels: Array<IEditorPanel>, side: TEditorPanelSide): Array<IEditorPanel> {
  return panels.filter((panel: IEditorPanel) => (panel.side ?? "right") === side);
}

/**
 * Publish the panels this application offers, for as long as it is mounted.
 */
export function useEditorPanels(panels: Array<IEditorPanel>): void {
  const { setPanels } = useContext(EditorPanelsContext);

  const latest = useRef<Array<IEditorPanel>>(panels);
  const key: string = panels
    .map((panel) => `${panel.id}:${panel.label}:${panel.side ?? "right"}:${panel.isOpenByDefault !== false}`)
    .join("|");

  latest.current = panels;

  useEffect(() => {
    setPanels(latest.current);

    return () => setPanels([]);
  }, [key, setPanels]);
}

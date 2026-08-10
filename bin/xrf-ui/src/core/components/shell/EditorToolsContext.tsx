import { createContext, ReactElement, ReactNode, useContext, useEffect, useMemo, useRef, useState } from "react";

export interface IEditorTool {
  id: string;
  label: string;
  icon: ReactNode;
  isOpenByDefault?: boolean;
  render: () => ReactNode;
}

interface IEditorToolsContextValue {
  tools: Array<IEditorTool>;
  setTools: (tools: Array<IEditorTool>) => void;
}

const EditorToolsContext = createContext<IEditorToolsContextValue>({
  tools: [],
  setTools: () => {},
});

export function EditorToolsProvider({ children }: { children: ReactNode }): ReactElement {
  const [tools, setTools] = useState<Array<IEditorTool>>([]);

  const value: IEditorToolsContextValue = useMemo(() => ({ tools, setTools }), [tools]);

  return <EditorToolsContext.Provider value={value}>{children}</EditorToolsContext.Provider>;
}

export function useEditorToolsRegistry(): Array<IEditorTool> {
  return useContext(EditorToolsContext).tools;
}

/**
 * Publish the tool panels this editor offers, for as long as it is mounted.
 *
 * Deliberately the same shape as `useEditorStatus`: an editor declares what it has, the shell decides
 * where it goes, and everything clears on unmount. Tools are compared by id and label rather than by
 * array identity, so callers can declare them inline without memoising.
 */
export function useEditorTools(tools: Array<IEditorTool>): void {
  const { setTools } = useContext(EditorToolsContext);

  const latest = useRef<Array<IEditorTool>>(tools);
  const key: string = tools.map((tool) => `${tool.id}:${tool.label}:${tool.isOpenByDefault !== false}`).join("|");

  latest.current = tools;

  useEffect(() => {
    setTools(latest.current);

    return () => setTools([]);
  }, [key, setTools]);
}

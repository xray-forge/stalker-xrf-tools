import { createContext, ReactElement, ReactNode, useContext, useEffect, useMemo, useState } from "react";

interface IEditorBusyContextValue {
  isBusy: boolean;
  setBusy: (isBusy: boolean) => void;
}

const EditorBusyContext = createContext<IEditorBusyContextValue>({
  isBusy: false,
  setBusy: () => {},
});

export function EditorBusyProvider({ children }: { children: ReactNode }): ReactElement {
  const [isBusy, setBusy] = useState<boolean>(false);

  const value: IEditorBusyContextValue = useMemo(() => ({ isBusy, setBusy }), [isBusy]);

  return <EditorBusyContext.Provider value={value}>{children}</EditorBusyContext.Provider>;
}

export function useIsEditorBusy(): boolean {
  return useContext(EditorBusyContext).isBusy;
}

/**
 * Declare that the active editor is running something that should not be walked away from.
 *
 * The shell blocks navigation while this is set. Forms already disable their own submit and back
 * control during a command, but the rail sits outside the route and happily navigated away mid-unpack,
 * leaving the operation running against a screen nobody could see.
 *
 * Clears on unmount, like the other editor-published state, so a crash or a route change cannot strand
 * the application in a permanently blocked state.
 */
export function useEditorBusy(isBusy: boolean): void {
  const { setBusy } = useContext(EditorBusyContext);

  useEffect(() => {
    setBusy(isBusy);

    return () => setBusy(false);
  }, [isBusy, setBusy]);
}

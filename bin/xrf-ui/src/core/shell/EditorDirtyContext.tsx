import { createContext, ReactElement, ReactNode, useCallback, useContext, useEffect, useMemo, useState } from "react";

import { ConfirmDialog } from "@/core/ui/dialog/ConfirmDialog";
import { Nullable } from "@/lib/types/general";

interface IEditorDirtyContextValue {
  dirtyCount: number;
  setDirtyCount: (dirtyCount: number) => void;
  requestLeave: (leave: () => void) => void;
}

const EditorDirtyContext = createContext<IEditorDirtyContextValue>({
  dirtyCount: 0,
  setDirtyCount: () => {},
  requestLeave: (leave: () => void) => leave(),
});

/**
 * @returns How many of the active editor's files hold edits that are not on disk.
 */
export function useEditorDirtyCount(): number {
  return useContext(EditorDirtyContext).dirtyCount;
}

/**
 * Runs an action that would abandon the active editor, asking first when work would be lost.
 */
export function useRequestLeave(): (leave: () => void) => void {
  return useContext(EditorDirtyContext).requestLeave;
}

/**
 * Publishes how much unsaved work the active editor is holding.
 *
 * @param dirtyCount - Number of files holding edits that have not been written.
 */
export function useEditorDirty(dirtyCount: number): void {
  const { setDirtyCount } = useContext(EditorDirtyContext);

  useEffect(() => {
    setDirtyCount(dirtyCount);

    return () => setDirtyCount(0);
  }, [dirtyCount, setDirtyCount]);
}

export function EditorDirtyProvider({ children }: { children: ReactNode }): ReactElement {
  const [dirtyCount, setDirtyCount] = useState<number>(0);
  const [pendingLeave, setPendingLeave] = useState<Nullable<() => void>>(null);

  const requestLeave = useCallback(
    (leave: () => void) => {
      if (dirtyCount > 0) {
        // Stored as a thunk, so `useState` invokes nothing while it holds the callback.
        setPendingLeave(() => leave);
      } else {
        leave();
      }
    },
    [dirtyCount]
  );

  const value: IEditorDirtyContextValue = useMemo(
    () => ({ dirtyCount, setDirtyCount, requestLeave }),
    [dirtyCount, requestLeave]
  );

  const onConfirm = useCallback(() => {
    pendingLeave?.();
    setPendingLeave(null);
  }, [pendingLeave]);

  const onClose = useCallback(() => {
    setPendingLeave(null);
  }, []);

  return (
    <EditorDirtyContext.Provider value={value}>
      {children}

      <ConfirmDialog
        isDestructive={true}
        isOpen={pendingLeave !== null}
        title={"Leave without saving?"}
        description={
          `${dirtyCount} ${dirtyCount === 1 ? "file has" : "files have"} edits that are not written to disk. ` +
          "Leaving discards them."
        }
        confirmLabel={"Discard and leave"}
        cancelLabel={"Stay"}
        onConfirm={onConfirm}
        onClose={onClose}
      />
    </EditorDirtyContext.Provider>
  );
}

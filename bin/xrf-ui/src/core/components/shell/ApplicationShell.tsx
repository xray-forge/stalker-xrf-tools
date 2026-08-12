import { ReactElement, ReactNode } from "react";

import { ApplicationShellFrame } from "@/core/components/shell/ApplicationShellFrame";
import { EditorBusyProvider } from "@/core/components/shell/EditorBusyContext";
import { EditorStatusProvider } from "@/core/components/shell/EditorStatusContext";
import { EditorPanelsProvider } from "@/core/components/shell/panel/EditorPanelsContext";

export interface IApplicationShellProps {
  children: ReactNode;
}

/**
 * Supplies the shell's contexts and renders the frame that consumes them.
 *
 * Split so the frame can read those contexts: a provider cannot consume what it provides.
 */
export function ApplicationShell({ children }: IApplicationShellProps): ReactElement {
  return (
    <EditorStatusProvider>
      <EditorPanelsProvider>
        <EditorBusyProvider>
          <ApplicationShellFrame>{children}</ApplicationShellFrame>
        </EditorBusyProvider>
      </EditorPanelsProvider>
    </EditorStatusProvider>
  );
}

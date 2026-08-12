import { ReactElement } from "react";

import { EditorToolbar } from "@/core/components/editor/EditorToolbar";
import { DelayedProgress } from "@/core/components/layout/DelayedProgress";
import { BaseComponentProps } from "@/lib/dom/element-types";

import { EditorLayout } from "./editor/EditorLayout";

/**
 * Shown while an application is still arriving - its chunk fetched, or its services provisioning.
 */
export function ApplicationLoader({ "data-testid": dataTestId, id, className }: BaseComponentProps): ReactElement {
  return (
    <EditorLayout toolbar={<EditorToolbar backPath={"/"} isBackDisabled={true} />}>
      <DelayedProgress data-testid={dataTestId} id={id} className={className} />
    </EditorLayout>
  );
}

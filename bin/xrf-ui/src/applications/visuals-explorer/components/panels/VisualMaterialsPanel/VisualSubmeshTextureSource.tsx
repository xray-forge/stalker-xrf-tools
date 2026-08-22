import { ReactElement } from "react";

import { VisualPanelRow } from "@/applications/visuals-explorer/components/panels/VisualPanelRow";
import { XrayAssetContainer } from "@/core/bindings/types/xrf-vfs";
import { BaseComponentProps } from "@/lib/dom/element-types";

export interface IVisualSubmeshTextureSourceProps extends BaseComponentProps {
  container: XrayAssetContainer;
}

/**
 * Displays the physical source of a resolved texture.
 *
 * Directory entries show their root and relative path. Archive entries show the volume-set path instead of inventing a
 * filesystem path for the entry.
 */
export function VisualSubmeshTextureSource({ container }: IVisualSubmeshTextureSourceProps): ReactElement {
  if (container.kind === "archive") {
    return (
      <>
        <VisualPanelRow label={"Source"} value={"Archive"} />
        <VisualPanelRow label={"Archive"} value={container.path} />
      </>
    );
  }

  return (
    <>
      <VisualPanelRow label={"Root"} value={container.root} />
      <VisualPanelRow label={"File"} value={container.relativePath} />
    </>
  );
}

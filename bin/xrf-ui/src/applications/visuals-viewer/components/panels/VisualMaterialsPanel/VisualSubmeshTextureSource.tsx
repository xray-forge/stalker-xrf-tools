import { ReactElement } from "react";

import { VisualPanelRow } from "@/applications/visuals-viewer/components/panels/VisualPanelRow";
import { XrayAssetContainer } from "@/core/bindings/xrf-assets";

export interface IVisualSubmeshTextureSourceProps {
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

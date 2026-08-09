import { Box } from "@mui/material";
import { ReactElement, useEffect, useRef } from "react";

import { Optional } from "@/core/types/general";
import { createStubVisualMeshData, IVisualPreviewViewOptions, VisualPreviewScene } from "@/lib/visuals";

interface IVisualPreviewViewportProps {
  options: IVisualPreviewViewOptions;
  cameraResetToken: number;
}

/**
 * Mounts the imperative preview scene and disposes it on unmount.
 *
 * The scene is created per mount rather than kept in state, so react strict mode remounting rebuilds a
 * clean webgl context instead of leaking the previous one. View options are read through a ref on mount
 * so a remount restores whatever the toolbar currently shows.
 */
export function VisualPreviewViewport({ options, cameraResetToken }: IVisualPreviewViewportProps): ReactElement {
  const containerRef = useRef<HTMLDivElement>(null);
  const sceneRef = useRef<Optional<VisualPreviewScene>>(null);
  const optionsRef = useRef<IVisualPreviewViewOptions>(options);

  useEffect(() => {
    if (!containerRef.current) {
      return;
    }

    const scene: VisualPreviewScene = new VisualPreviewScene(createStubVisualMeshData());

    sceneRef.current = scene;

    scene.mount(containerRef.current);
    scene.applyViewOptions(optionsRef.current);

    return () => {
      sceneRef.current = null;
      scene.dispose();
    };
  }, []);

  useEffect(() => {
    optionsRef.current = options;
    sceneRef.current?.applyViewOptions(options);
  }, [options]);

  useEffect(() => {
    sceneRef.current?.resetCamera();
  }, [cameraResetToken]);

  return <Box ref={containerRef} sx={{ width: "100%", height: "100%", overflow: "hidden" }} />;
}

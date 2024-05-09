import { clamp } from "@mui/x-data-grid/internals";
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { ContextManager, createActions, createLoadable, Loadable } from "dreamstate";

import { Optional } from "@/core/types/general";
import { blobToImage } from "@/lib/image";
import { EIconsEditorCommand } from "@/lib/ipc";
import { Logger } from "@/lib/logging";

export interface IEquipmentPngDescriptor {
  path: string;
  uri: string;
  blob: Blob;
  image: HTMLImageElement;
}

export interface IEquipmentContext {
  equipmentActions: {
    open(spritePath: string, systemLtxPath: string): Promise<void>;
    setGridVisibility(isVisible: boolean): void;
    setGridSize(size: number): void;
  };
  gridSize: number;
  isGridVisible: boolean;
  spriteImage: Loadable<Optional<IEquipmentPngDescriptor>>;
}

export class EquipmentManager extends ContextManager<IEquipmentContext> {
  public context: IEquipmentContext = {
    equipmentActions: createActions({
      open: (spritePath: string, systemLtxPath: string) => this.openEquipmentProject(spritePath, systemLtxPath),
      setGridVisibility: (isVisible: boolean) => this.setContext({ isGridVisible: isVisible }),
      setGridSize: (size: number) => this.setContext({ gridSize: Math.round(clamp(size, 10, 100)) }),
    }),
    gridSize: 50,
    isGridVisible: true,
    spriteImage: createLoadable(null),
  };

  public log: Logger = new Logger("equipment");

  public async openEquipmentProject(spritePath: string, systemLtxPath: string): Promise<void> {
    this.log.info("Opening equipment project:", spritePath, systemLtxPath);

    try {
      this.setContext({ spriteImage: createLoadable(null, true) });

      const imageUri: string = await invoke(EIconsEditorCommand.GET_EQUIPMENT_SPRITE_URI, {
        spritePath,
        systemLtxPath,
      });

      const blob: Blob = await fetch(convertFileSrc(imageUri, "stream")).then((response) => response.blob());
      const image: HTMLImageElement = await blobToImage(blob);

      this.log.info("Equipment project opened:", imageUri);

      this.setContext({
        spriteImage: createLoadable({
          path: spritePath,
          blob,
          image,
          uri: imageUri,
        }),
      });
    } catch (error) {
      this.log.error("Failed to open equipment project:", error);
      this.setContext({ spriteImage: createLoadable(null, false, error as Error) });
    }
  }
}

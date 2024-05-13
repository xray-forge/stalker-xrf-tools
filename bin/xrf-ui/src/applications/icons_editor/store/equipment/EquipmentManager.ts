import { clamp } from "@mui/x-data-grid/internals";
import { path } from "@tauri-apps/api";
import { exists } from "@tauri-apps/api/fs";
import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
import { ContextManager, createActions, createLoadable, Loadable } from "dreamstate";

import { Optional } from "@/core/types/general";
import { IEquipmentResponse, IEquipmentSectionDescriptor, IPackEquipmentResult } from "@/lib/icons";
import { blobToImage } from "@/lib/image";
import { EIconsEditorCommand } from "@/lib/ipc";
import { Logger } from "@/lib/logging";

export interface IEquipmentPngDescriptor {
  ltxPath: string;
  descriptors: Array<IEquipmentSectionDescriptor>;
  path: string;
  name: string;
  blob: Blob;
  image: HTMLImageElement;
}

export interface IEquipmentContext {
  equipmentActions: {
    open(spritePath: string, systemLtxPath: string): Promise<void>;
    pack(sourcePath: string, outputPath: string, systemLtxPath: string): Promise<IPackEquipmentResult>;
    reopen(): Promise<void>;
    repackAndOpen(): Promise<void>;
    close(): Promise<void>;
    setGridVisibility(isVisible: boolean): void;
    setGridSize(size: number): void;
  };
  isReady: boolean;
  isGridVisible: boolean;
  gridSize: number;
  spriteImage: Loadable<Optional<IEquipmentPngDescriptor>>;
}

export class EquipmentManager extends ContextManager<IEquipmentContext> {
  public context: IEquipmentContext = {
    equipmentActions: createActions({
      open: (spritePath: string, systemLtxPath: string) => this.openEquipmentProject(spritePath, systemLtxPath),
      pack: (sourcePath: string, outputPath: string, systemLtxPath: string) =>
        this.packEquipmentSprite(sourcePath, outputPath, systemLtxPath),
      reopen: () => this.reopenEquipmentProject(),
      repackAndOpen: () => this.repackAndOpenProject(),
      close: () => this.closeEquipmentProject(),
      setGridVisibility: (isVisible: boolean) => this.setContext({ isGridVisible: isVisible }),
      setGridSize: (size: number) => this.setContext({ gridSize: Math.round(clamp(size, 10, 100)) }),
    }),
    gridSize: 50,
    isReady: false,
    isGridVisible: true,
    spriteImage: createLoadable(null),
  };

  public log: Logger = new Logger("equipment_editor");

  public async onProvisionStarted(): Promise<void> {
    const response: IEquipmentResponse = await invoke(EIconsEditorCommand.GET_EQUIPMENT_SPRITE);

    if (response) {
      this.log.info("Existing equipment_editor sprite detected");

      this.setContext({
        isReady: true,
        spriteImage: createLoadable(await this.spriteFromResponse(response)),
      });
    } else {
      this.log.info("No existing sprite detected file");
      this.setContext({ isReady: true });
    }
  }

  public async openEquipmentProject(equipmentDdsPath: string, systemLtxPath: string): Promise<void> {
    this.log.info("Opening equipment_editor project:", equipmentDdsPath, systemLtxPath);

    try {
      this.cleanupAssets();
      this.setContext({ spriteImage: createLoadable(null, true) });

      const response: IEquipmentResponse = await invoke(EIconsEditorCommand.OPEN_EQUIPMENT_SPRITE, {
        equipmentDdsPath,
        systemLtxPath,
      });

      this.log.info("Equipment project opened:", response);

      this.setContext({
        spriteImage: createLoadable(await this.spriteFromResponse(response)),
      });
    } catch (error) {
      this.log.error("Failed to open equipment editor project:", error);
      this.setContext({ spriteImage: createLoadable(null, false, error as Error) });
    }
  }

  public async reopenEquipmentProject(): Promise<void> {
    this.log.info("Reopening equipment editor project");

    try {
      this.setContext(({ spriteImage }) => ({ spriteImage: spriteImage.asLoading() }));

      const response: IEquipmentResponse = await invoke(EIconsEditorCommand.REOPEN_EQUIPMENT_SPRITE);

      this.log.info("Equipment project reopened:", response);

      this.cleanupAssets();
      this.setContext({
        spriteImage: createLoadable(await this.spriteFromResponse(response)),
      });
    } catch (error) {
      this.log.error("Failed to reopen equipment editor project:", error);
      throw error;
    }
  }

  public async repackAndOpenProject(): Promise<void> {
    const { spriteImage } = this.context;

    if (!spriteImage.value || spriteImage.isLoading) {
      throw new Error("Invalid attempt to reopen project that is loading or not open.");
    }

    this.log.info("Repack and reopen equipment editor project");

    const inputPath: string = await path.join(
      await path.dirname(spriteImage.value.path),
      await path.basename(spriteImage.value.path, await path.extname(spriteImage.value.path))
    );

    if (!(await exists(inputPath))) {
      throw new Error(`Invalid attempt to repack DDS without base icons in '${inputPath}'.`);
    }

    try {
      this.setContext(({ spriteImage }) => ({ spriteImage: spriteImage.asLoading() }));

      await this.packEquipmentSprite(inputPath, spriteImage.value.path, spriteImage.value.ltxPath);

      await this.reopenEquipmentProject();
    } finally {
      if (this.context.spriteImage.isLoading) {
        this.setContext(({ spriteImage }) => ({ spriteImage: spriteImage.asReady() }));
      }
    }
  }

  public async closeEquipmentProject(): Promise<void> {
    this.log.info("Closing equipment_editor");

    try {
      this.setContext(({ spriteImage }) => ({ spriteImage: spriteImage.asLoading() }));
      this.cleanupAssets();

      await invoke(EIconsEditorCommand.CLOSE_EQUIPMENT_SPRITE);

      this.log.info("Equipment project closed");

      this.setContext({ spriteImage: createLoadable(null) });
    } catch (error) {
      this.log.error("Failed to close equipment editor project:", error);
      this.setContext(({ spriteImage }) => ({ spriteImage: spriteImage.asFailed(new Error(error as string)) }));
    }
  }

  public async packEquipmentSprite(
    sourcePath: string,
    outputPath: string,
    systemLtxPath: string
  ): Promise<IPackEquipmentResult> {
    this.log.info("Packing equipment editor:", sourcePath, outputPath, systemLtxPath);

    try {
      return await invoke(EIconsEditorCommand.PACK_EQUIPMENT, {
        sourcePath,
        outputPath,
        systemLtxPath,
      });
    } catch (error) {
      this.log.error("Failed to pack equipment editor:", error);
      throw error;
    }
  }

  public async spriteFromResponse(response: IEquipmentResponse): Promise<IEquipmentPngDescriptor> {
    const blob: Blob = await fetch(convertFileSrc(response.name, "stream")).then((response) => response.blob());

    return {
      blob,
      ltxPath: response.systemLtxPath,
      descriptors: response.equipmentDescriptors,
      image: await blobToImage(blob),
      name: response.name,
      path: response.path,
    };
  }

  public cleanupAssets(): void {
    const { spriteImage } = this.context;

    if (spriteImage.value) {
      URL.revokeObjectURL(spriteImage.value.image.src);
    }
  }
}

import { clamp } from "@mui/x-data-grid/internals";
import { path } from "@tauri-apps/api";
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { exists } from "@tauri-apps/plugin-fs";
import { OnProvision } from "@wirestate/core";
import { BoundAction, makeObservable, Observable } from "@wirestate/react-mobx";

import { Optional } from "@/core/types/general";
import { IEquipmentResponse, IEquipmentSectionDescriptor, IPackEquipmentResult } from "@/lib/icons";
import { blobToImage } from "@/lib/image";
import { EIconsEditorCommand } from "@/lib/ipc";
import { createLoadable, Loadable } from "@/lib/loadable";
import { Logger } from "@/lib/logging";

export interface IEquipmentPngDescriptor {
  ltxPath: string;
  descriptors: Array<IEquipmentSectionDescriptor>;
  path: string;
  name: string;
  blob: Blob;
  image: HTMLImageElement;
}

export class EquipmentManager {
  @Observable()
  public gridSize: number = 50;

  @Observable()
  public isReady: boolean = false;

  @Observable()
  public isGridVisible: boolean = true;

  @Observable()
  public spriteImage: Loadable<Optional<IEquipmentPngDescriptor>> = createLoadable(null);

  public readonly log: Logger = new Logger(this.constructor.name);

  public constructor() {
    makeObservable(this);
  }

  @OnProvision()
  public async onProvision(): Promise<void> {
    const response: IEquipmentResponse = await invoke(EIconsEditorCommand.GET_EQUIPMENT_SPRITE);

    if (response) {
      this.log.info("Existing equipment sprite detected");
      this.isReady = true;
      this.spriteImage = createLoadable(await this.spriteFromResponse(response));
    } else {
      this.log.info("No existing sprite detected file");
      this.isReady = true;
    }
  }

  @BoundAction()
  public setGridVisibility(isVisible: boolean): void {
    this.isGridVisible = isVisible;
  }

  @BoundAction()
  public setGridSize(size: number): void {
    this.gridSize = Math.round(clamp(size, 10, 100));
  }

  @BoundAction()
  public async openEquipmentProject(equipmentDdsPath: string, systemLtxPath: string): Promise<void> {
    this.log.info("Opening equipment project:", equipmentDdsPath, systemLtxPath);

    try {
      this.cleanupAssets();
      this.spriteImage = createLoadable(null, true);

      const response: IEquipmentResponse = await invoke(EIconsEditorCommand.OPEN_EQUIPMENT_SPRITE, {
        equipmentDdsPath,
        systemLtxPath,
      });

      this.log.info("Equipment project opened:", response);

      this.spriteImage = createLoadable(await this.spriteFromResponse(response));
    } catch (error) {
      this.log.error("Failed to open equipment editor project:", error);
      this.spriteImage = createLoadable(null, false, error as Error);
    }
  }

  @BoundAction()
  public async reopenEquipmentProject(): Promise<void> {
    this.log.info("Reopening equipment editor project");

    try {
      this.spriteImage = this.spriteImage.asLoading();

      const response: IEquipmentResponse = await invoke(EIconsEditorCommand.REOPEN_EQUIPMENT_SPRITE);

      this.log.info("Equipment project reopened:", response);

      this.cleanupAssets();
      this.spriteImage = createLoadable(await this.spriteFromResponse(response));
    } catch (error) {
      this.log.error("Failed to reopen equipment editor project:", error);
      throw error;
    }
  }

  @BoundAction()
  public async repackAndOpenProject(): Promise<void> {
    const { spriteImage } = this;

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
      this.spriteImage = this.spriteImage.asLoading();

      await this.packEquipmentSprite(inputPath, spriteImage.value.path, spriteImage.value.ltxPath);

      await this.reopenEquipmentProject();
    } finally {
      if (this.spriteImage.isLoading) {
        this.spriteImage = this.spriteImage.asReady();
      }
    }
  }

  @BoundAction()
  public async closeEquipmentProject(): Promise<void> {
    this.log.info("Closing equipment project");

    try {
      this.spriteImage = this.spriteImage.asLoading();
      this.cleanupAssets();

      await invoke(EIconsEditorCommand.CLOSE_EQUIPMENT_SPRITE);

      this.log.info("Equipment project closed");

      this.spriteImage = createLoadable(null);
    } catch (error) {
      this.log.error("Failed to close equipment editor project:", error);
      this.spriteImage = this.spriteImage.asFailed(new Error(error as string));
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
    if (this.spriteImage.value) {
      URL.revokeObjectURL(this.spriteImage.value.image.src);
    }
  }
}

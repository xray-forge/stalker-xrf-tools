import { beforeEach, describe, expect, it } from "@jest/globals";

import { ArchivesService } from "@/applications/archives-explorer/services/archives/archives.service";
import { ArchiveFileDescriptor } from "@/core/bindings/types/xrf-archive";
import { mockArchiveFileDescriptor, mockArchivesProject } from "@/fixtures/mocks/archive.mocks";
import { mockInvoke, setMockInvokeResponses } from "@/fixtures/mocks/tauri.mocks";
import { mockInjectedService } from "@/fixtures/utils/container";
import { createLoadable } from "@/lib/loadable";

const SOUND: ArchiveFileDescriptor = mockArchiveFileDescriptor({
  extension: "ogg",
  name: "sounds\\ambient\\wind.ogg",
  sizeCompressed: 4096,
  sizeReal: 8192,
});

const TEXTURE: ArchiveFileDescriptor = mockArchiveFileDescriptor({ extension: "dds", name: "textures\\ui.dds" });

const PREVIEW = {
  name: SOUND.name,
  channels: 2,
  sampleRate: 44100,
  parameters: { minDistance: 1, maxDistance: 50, baseVolume: 0.8, gameType: 3, maxAiDistance: 25 },
  base64: "T2dnUw==",
};

function createService(): ArchivesService {
  const { service } = mockInjectedService(ArchivesService);

  service.project = createLoadable(mockArchivesProject([SOUND, TEXTURE]));

  return service;
}

describe("ArchivesService audio preview", () => {
  beforeEach(() => {
    setMockInvokeResponses({ ["plugin:archives|read_audio"]: PREVIEW });
  });

  it("routes a sound to the audio command rather than reading it as text", async () => {
    const service: ArchivesService = createService();

    await service.selectArchiveFile(SOUND);

    expect(mockInvoke).toHaveBeenCalledWith("plugin:archives|read_audio", { path: SOUND.name });
    expect(mockInvoke).not.toHaveBeenCalledWith("plugin:archives|read_file", expect.anything());
    expect(service.content.value?.kind).toBe("audio");
  });

  it("carries the engine parameters the archive stored", async () => {
    const service: ArchivesService = createService();

    await service.selectArchiveFile(SOUND);

    // These come from the vorbis comment and are the reason the backend parses at all - the webview
    // could play the bytes without any of it.
    expect(service.content.value?.kind === "audio" ? service.content.value.preview.parameters : null).toEqual(
      PREVIEW.parameters
    );
  });

  it("keeps textures on the image path", async () => {
    const service: ArchivesService = createService();

    await service.selectArchiveFile(TEXTURE);

    expect(mockInvoke).not.toHaveBeenCalledWith("plugin:archives|read_audio", expect.anything());
    expect(service.content.value?.kind).toBe("image");
  });

  it("reports a failed read instead of staying loading", async () => {
    const service: ArchivesService = createService();

    setMockInvokeResponses({
      ["plugin:archives|read_audio"]: () => {
        throw new Error("not a playable sound");
      },
    });

    await service.selectArchiveFile(SOUND);

    expect(service.content.isLoading).toBe(false);
    expect(String(service.content.error)).toContain("not a playable sound");
  });

  it("retries the audio read rather than falling back to text", async () => {
    const service: ArchivesService = createService();

    await service.selectArchiveFile(SOUND);
    await service.retrySelectedFile();

    const audioCalls = mockInvoke.mock.calls.filter(([command]) => command === "plugin:archives|read_audio");

    expect(audioCalls).toHaveLength(2);
  });
});

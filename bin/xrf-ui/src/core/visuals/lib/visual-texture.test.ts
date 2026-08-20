import { describe, expect, it } from "@jest/globals";
import {
  CompressedTexture,
  LinearFilter,
  RepeatWrapping,
  RGB_S3TC_DXT1_Format,
  RGBA_S3TC_DXT5_Format,
} from "three";

import { SubmeshTexture } from "@/core/bindings/xrf-app-visuals";
import { XrayAsset } from "@/core/bindings/xrf-vfs";
import {
  createDdsTexture,
  EVisualTextureState,
  isLoadableResolution,
  toInitialTextureState,
  toLoadableTextures,
} from "@/core/visuals/lib/visual-texture";
import { mockDdsFile, mockDx10DdsFile, mockUncompressedDdsFile } from "@/fixtures/mocks/dds.mocks";
import { mockSubmeshTexture } from "@/fixtures/mocks/visual.mocks";
import { Nullable } from "@/lib/types/general";

describe("isLoadableResolution", () => {
  it("counts a substituted reference as loadable", () => {
    // The engine's dummy is a real file and rendering it is what the game does, so it is fetched like any other.
    const location: XrayAsset = {
      container: {
        kind: "directory",
        relativePath: "textures\\ed\\ed_not_existing_texture.dds",
        root: "C:\\gamedata",
      },
      logicalPath: "textures\\ed\\ed_not_existing_texture.dds",
    };

    expect(isLoadableResolution({ kind: "resolved", location })).toBe(true);
    expect(isLoadableResolution({ kind: "substituted", location })).toBe(true);
  });

  it("counts every outcome without a file as not loadable", () => {
    expect(isLoadableResolution({ kind: "none" })).toBe(false);
    expect(isLoadableResolution({ kind: "noRoot" })).toBe(false);
    expect(isLoadableResolution({ kind: "missing", roots: ["C:\\gamedata"] })).toBe(false);
  });
});

describe("toLoadableTextures", () => {
  it("keeps only submeshes with both a reference and a located file", () => {
    const textures: Array<SubmeshTexture> = [
      mockSubmeshTexture({ submeshIndex: 0 }),
      mockSubmeshTexture({ reference: null, resolution: { kind: "none" }, submeshIndex: 1 }),
      mockSubmeshTexture({ resolution: { kind: "missing", roots: ["C:\\gamedata"] }, submeshIndex: 2 }),
      mockSubmeshTexture({ resolution: { kind: "noRoot" }, submeshIndex: 3 }),
    ];

    expect(toLoadableTextures(textures).map((it) => it.submeshIndex)).toEqual([0]);
  });
});

describe("toInitialTextureState", () => {
  it("separates a submesh with no texture from one whose texture was not found", () => {
    // Both end up untextured, but only one of them is a problem, so the panel must not report them the same way.
    expect(toInitialTextureState(mockSubmeshTexture({ reference: null, resolution: { kind: "none" } }))).toBe(
      EVisualTextureState.ABSENT
    );
    expect(toInitialTextureState(mockSubmeshTexture({ resolution: { kind: "noRoot" } }))).toBe(
      EVisualTextureState.UNRESOLVED
    );
    expect(toInitialTextureState(mockSubmeshTexture())).toBe(EVisualTextureState.LOADING);
  });
});

describe("createDdsTexture", () => {
  it("uploads a dxt1 file with its mip chain", () => {
    const texture: Nullable<CompressedTexture> = createDdsTexture(
      mockDdsFile({ fourCC: "DXT1", height: 4, mipmapCount: 3, width: 4 })
    );

    expect(texture).not.toBeNull();
    expect(texture!.format).toBe(RGB_S3TC_DXT1_Format);
    expect(texture!.image.width).toBe(4);
    expect(texture!.mipmaps).toHaveLength(3);
  });

  it("drops to a non mipmap filter when the file carries no mip chain", () => {
    // Load bearing rather than cosmetic: webgl samples an incomplete texture as black, and most modded textures ship
    // without mips - 1,805 of Anomaly's 2,197.
    const withMips: Nullable<CompressedTexture> = createDdsTexture(mockDdsFile({ mipmapCount: 4 }));
    const withoutMips: Nullable<CompressedTexture> = createDdsTexture(mockDdsFile({ mipmapCount: 1 }));

    expect(withoutMips!.minFilter).toBe(LinearFilter);
    expect(withMips!.minFilter).not.toBe(LinearFilter);
  });

  it("samples with wrap addressing, as the engine does", () => {
    // `r_Sampler` defaults to `D3DTADDRESS_WRAP` and the model blender does not override it. three.js defaults to
    // clamp, which smears the edge texel over every face whose uv leaves [0,1] - `wpn_colt1911` reaches u = -0.997.
    const texture: Nullable<CompressedTexture> = createDdsTexture(mockDdsFile());

    expect(texture!.wrapS).toBe(RepeatWrapping);
    expect(texture!.wrapT).toBe(RepeatWrapping);
  });

  it("uploads a dxt5 file", () => {
    expect(createDdsTexture(mockDdsFile({ fourCC: "DXT5" }))!.format).toBe(RGBA_S3TC_DXT5_Format);
  });

  it("refuses a bc7 file rather than uploading garbage", () => {
    // Gunslinger ships three of these. The loader logs its own complaint and returns a parse with no format.
    expect(createDdsTexture(mockDx10DdsFile(98))).toBeNull();
  });

  it("refuses an rgba ordered uncompressed file, which the loader only accepts as bgra", () => {
    // Anomaly ships 24 references to `A8B8G8R8`. Its red channel sits in the low byte, and the loader tests for red in
    // `0x00ff0000`, so it matches neither uncompressed branch.
    const bgra: ArrayBuffer = mockUncompressedDdsFile({ blueMask: 0x000000ff, redMask: 0x00ff0000 });
    const rgba: ArrayBuffer = mockUncompressedDdsFile({ blueMask: 0x00ff0000, redMask: 0x000000ff });

    expect(createDdsTexture(bgra)).not.toBeNull();
    expect(createDdsTexture(rgba)).toBeNull();
  });
});

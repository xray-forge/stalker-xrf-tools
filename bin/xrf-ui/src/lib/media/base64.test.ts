import { describe, expect, it } from "@jest/globals";

import { base64ToBlob, base64ToBytes } from "@/lib/media/base64";

describe("base64ToBytes", () => {
  it("preserves bytes above the ascii range", () => {
    // "OggS" followed by a high byte. Treating the binary string as text would encode 0xff as two
    // bytes and corrupt the container header this is used for.
    expect(Array.from(base64ToBytes("T2dnU/8="))).toEqual([0x4f, 0x67, 0x67, 0x53, 0xff]);
  });

  it("round trips every byte value", () => {
    const original: Uint8Array = new Uint8Array(256);

    for (let index = 0; index < 256; index += 1) {
      original[index] = index;
    }

    const encoded: string = btoa(String.fromCharCode(...original));

    expect(Array.from(base64ToBytes(encoded))).toEqual(Array.from(original));
  });

  it("decodes empty input to nothing", () => {
    expect(base64ToBytes("")).toHaveLength(0);
  });
});

describe("base64ToBlob", () => {
  it("carries the size and media type through", () => {
    const blob: Blob = base64ToBlob("T2dnU/8=", "audio/ogg");

    expect(blob.size).toBe(5);
    expect(blob.type).toBe("audio/ogg");
  });
});

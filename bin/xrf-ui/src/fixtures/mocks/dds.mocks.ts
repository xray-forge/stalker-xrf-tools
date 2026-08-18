/**
 * Minimal DDS files, built byte by byte.
 *
 * Offsets follow `three/examples/jsm/loaders/DDSLoader.js`, which reads the header as an `Int32Array` over the whole
 * buffer, so index 0 is the magic number and the pixel format begins at index 20.
 */

const DDS_MAGIC: number = 0x20534444;
/** Header size in bytes as the format declares it, which is also what the loader adds 4 to for its data offset. */
const HEADER_SIZE: number = 124;
const DATA_OFFSET: number = HEADER_SIZE + 4;
/** Bytes the `DX10` extended header adds before pixel data. */
const DX10_HEADER_SIZE: number = 20;
/** `DDSD_MIPMAPCOUNT`, without which the loader ignores the mipmap count and assumes one. */
const FLAG_MIPMAP_COUNT: number = 0x20000;

const OFFSET_MAGIC: number = 0;
const OFFSET_SIZE: number = 1;
const OFFSET_FLAGS: number = 2;
const OFFSET_HEIGHT: number = 3;
const OFFSET_WIDTH: number = 4;
const OFFSET_MIPMAP_COUNT: number = 7;
const OFFSET_FOUR_CC: number = 21;
const OFFSET_RGB_BIT_COUNT: number = 22;
const OFFSET_RED_MASK: number = 23;
const OFFSET_GREEN_MASK: number = 24;
const OFFSET_BLUE_MASK: number = 25;
const OFFSET_ALPHA_MASK: number = 26;
/** First int past the header, where the `DX10` extended header begins. */
const OFFSET_DXGI_FORMAT: number = 32;

/** Bytes one compressed block occupies, by the fourCC that declares it. */
const BLOCK_BYTES: Record<string, number> = {
  DX10: 16,
  DXT1: 8,
  DXT3: 16,
  DXT5: 16,
};

export interface IMockDdsOptions {
  fourCC?: keyof typeof BLOCK_BYTES;
  width?: number;
  height?: number;
  mipmapCount?: number;
}

export interface IMockUncompressedDdsOptions {
  width?: number;
  height?: number;
  redMask?: number;
  blueMask?: number;
}

/** A fourCC as the little-endian int the header stores. */
function toFourCC(fourCC: string): number {
  return (
    fourCC.charCodeAt(0) | (fourCC.charCodeAt(1) << 8) | (fourCC.charCodeAt(2) << 16) | (fourCC.charCodeAt(3) << 24)
  );
}

/**
 * Bytes a compressed mip chain occupies, by the loader's own arithmetic.
 *
 * Sized rather than guessed because the loader builds each mip as a view over the buffer, so a buffer one byte short
 * throws instead of returning a bad texture.
 */
function toCompressedDataSize(width: number, height: number, mipmapCount: number, blockBytes: number): number {
  let size: number = 0;
  let mipWidth: number = width;
  let mipHeight: number = height;

  for (let mip: number = 0; mip < mipmapCount; mip += 1) {
    size += (Math.max(4, mipWidth) / 4) * (Math.max(4, mipHeight) / 4) * blockBytes;

    mipWidth = Math.max(mipWidth >> 1, 1);
    mipHeight = Math.max(mipHeight >> 1, 1);
  }

  return size;
}

/**
 * A DDS file carrying a block-compressed format.
 *
 * @param options - Format, dimensions and mip count to declare.
 * @returns The file as bytes.
 */
export function mockDdsFile(options: IMockDdsOptions = {}): ArrayBuffer {
  const { fourCC = "DXT1", width = 4, height = 4, mipmapCount = 1 } = options;

  const dataSize: number = toCompressedDataSize(width, height, mipmapCount, BLOCK_BYTES[fourCC]);
  const buffer: ArrayBuffer = new ArrayBuffer(DATA_OFFSET + dataSize);
  const header: Int32Array = new Int32Array(buffer);

  header[OFFSET_MAGIC] = DDS_MAGIC;
  header[OFFSET_SIZE] = HEADER_SIZE;
  header[OFFSET_FLAGS] = FLAG_MIPMAP_COUNT;
  header[OFFSET_HEIGHT] = height;
  header[OFFSET_WIDTH] = width;
  header[OFFSET_MIPMAP_COUNT] = mipmapCount;
  header[OFFSET_FOUR_CC] = toFourCC(fourCC);

  return buffer;
}

/**
 * A DDS file whose format is declared by a `DX10` extended header.
 *
 * @param dxgiFormat - `DXGI_FORMAT` code to declare, such as 98 for `BC7_UNORM`.
 * @returns The file as bytes.
 */
export function mockDx10DdsFile(dxgiFormat: number): ArrayBuffer {
  const dataSize: number = toCompressedDataSize(4, 4, 1, BLOCK_BYTES.DX10);
  const buffer: ArrayBuffer = new ArrayBuffer(DATA_OFFSET + DX10_HEADER_SIZE + dataSize);
  const header: Int32Array = new Int32Array(buffer);

  header[OFFSET_MAGIC] = DDS_MAGIC;
  header[OFFSET_SIZE] = HEADER_SIZE;
  header[OFFSET_HEIGHT] = 4;
  header[OFFSET_WIDTH] = 4;
  header[OFFSET_FOUR_CC] = toFourCC("DX10");
  header[OFFSET_DXGI_FORMAT] = dxgiFormat;

  return buffer;
}

/**
 * A DDS file storing uncompressed 32-bit pixels, with the channel order the masks say.
 *
 * The masks are the point: the same 32 bits per pixel is `A8R8G8B8` when red sits in `0x00ff0000` and `A8B8G8R8`
 * when it sits in `0x000000ff`, and the loader accepts only the first.
 *
 * @param options - Dimensions and the red and blue channel masks.
 * @returns The file as bytes.
 */
export function mockUncompressedDdsFile(options: IMockUncompressedDdsOptions = {}): ArrayBuffer {
  const { width = 4, height = 4, redMask = 0x00ff0000, blueMask = 0x000000ff } = options;

  const buffer: ArrayBuffer = new ArrayBuffer(DATA_OFFSET + width * height * 4);
  const header: Int32Array = new Int32Array(buffer);

  header[OFFSET_MAGIC] = DDS_MAGIC;
  header[OFFSET_SIZE] = HEADER_SIZE;
  header[OFFSET_HEIGHT] = height;
  header[OFFSET_WIDTH] = width;
  header[OFFSET_FOUR_CC] = 0;
  header[OFFSET_RGB_BIT_COUNT] = 32;
  header[OFFSET_RED_MASK] = redMask;
  header[OFFSET_GREEN_MASK] = 0x0000ff00;
  header[OFFSET_BLUE_MASK] = blueMask;
  header[OFFSET_ALPHA_MASK] = 0xff000000;

  return buffer;
}

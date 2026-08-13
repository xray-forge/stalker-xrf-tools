// Auto-generated rust bindings. Do not edit it manually.

import { invoke as __TAURI_INVOKE } from "@tauri-apps/api/core";

/** Commands */
export const commands = {
  checkFormatConfigsPath: (path: string) =>
    __TAURI_INVOKE<LtxProjectFormatResult>("plugin:configs-editor|check_format_configs_path", { path }),
  formatConfigsPath: (path: string) =>
    __TAURI_INVOKE<LtxProjectFormatResult>("plugin:configs-editor|format_configs_path", { path }),
  verifyConfigsPath: (path: string) =>
    __TAURI_INVOKE<LtxProjectVerifyResult>("plugin:configs-editor|verify_configs_path", { path }),
};

/* Types */
export type LtxProjectFormatResult = {
  duration: number;
  invalidFiles: number;
  toFormat: Array<string>;
  totalFiles: number;
  validFiles: number;
};

export type LtxProjectVerifyResult = {
  checkedFields: number;
  checkedSections: number;
  duration: number;
  errors: Array<XRayError>;
  invalidSections: number;
  skippedSections: number;
  totalFiles: number;
  totalSections: number;
  validSections: number;
};

/** Error while working with translation file */
export type XRayError =
  | ({
      Assertion: {
        message: string;
      };
    } & {
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      Asset: {
        message: string;
      };
    } & {
      Assertion?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      Convert: {
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      Format: {
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      Verify: {
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
    })
  | ({
      NotImplemented: {
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      Read: {
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      Unexpected: {
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      NotFound: {
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      Invalid: {
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      Parsing: {
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      Encoding: {
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      NoTerminator: {
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      UnknownLanguage: {
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      Verify?: never;
    })
  | ({
      InvalidSource: {
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      Serialization: {
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      TextureProcessing: {
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      ChunkNotEnded: {
        message: string;
        remaining: number;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      LtxParse: {
        line: number;
        col: number;
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      LtxScheme: {
        section: string;
        field: string;
        message: string;
        at: string | null;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      Generic: {
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      Serde: {
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      Io?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    })
  | ({
      Io: {
        message: string;
      };
    } & {
      Assertion?: never;
      Asset?: never;
      ChunkNotEnded?: never;
      Convert?: never;
      Encoding?: never;
      Format?: never;
      Generic?: never;
      Invalid?: never;
      InvalidSource?: never;
      LtxParse?: never;
      LtxScheme?: never;
      NoTerminator?: never;
      NotFound?: never;
      NotImplemented?: never;
      Parsing?: never;
      Read?: never;
      Serde?: never;
      Serialization?: never;
      TextureProcessing?: never;
      Unexpected?: never;
      UnknownLanguage?: never;
      Verify?: never;
    });

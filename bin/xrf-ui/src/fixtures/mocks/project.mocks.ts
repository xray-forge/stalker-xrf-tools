import { TCallableExportDescriptor } from "@/lib/exports";
import { IEquipmentSectionDescriptor } from "@/lib/icons";
import { ITranslationsProjectJson } from "@/lib/translations";
import { ArchiveDescriptor, ArchiveFileDescriptor, ArchiveProject } from "@/lib/xrf/bindings/xray-archive";
import { ExportDescriptor, ExportsProject } from "@/lib/xrf/bindings/xray-export";

import { mockArchiveReadPolicy } from "./archive.mocks";

/** Create an archive file fixture with optional field overrides. */
export function mockArchiveFile(overrides: Partial<ArchiveFileDescriptor> = {}): ArchiveFileDescriptor {
  return {
    crc: 123456,
    destination: "gamedata\\config\\system.ltx",
    extension: "ltx",
    name: "system.ltx",
    offset: 0,
    sizeCompressed: 512,
    sizeReal: 2048,
    source: "db\\db0",
    ...overrides,
  };
}

/** Create an archive descriptor fixture with optional field overrides. */
export function mockArchiveDescriptor(overrides: Partial<ArchiveDescriptor> = {}): ArchiveDescriptor {
  return {
    createdAt: null,
    modifiedAt: null,
    files: {},
    outputRootPath: "unpacked",
    path: "db\\db0",
    ...overrides,
  };
}

/** Create an archives project fixture with optional field overrides. */
export function mockArchivesProject(overrides: Partial<ArchiveProject> = {}): ArchiveProject {
  return {
    archives: [mockArchiveDescriptor(), mockArchiveDescriptor({ path: "db\\db1" })],
    files: {
      "config\\system.ltx": mockArchiveFile(),
      "config\\weapons\\wpn_ak74.ltx": mockArchiveFile({
        name: "wpn_ak74.ltx",
        destination: "gamedata\\config\\weapons\\wpn_ak74.ltx",
      }),
      "meshes\\dynamics\\weapons\\wpn_ak74.ogf": mockArchiveFile({
        extension: "ogf",
        name: "wpn_ak74.ogf",
        destination: "gamedata\\meshes\\dynamics\\weapons\\wpn_ak74.ogf",
      }),
    },
    readPolicy: mockArchiveReadPolicy(),
    root: "db",
    sizeReal: 6144,
    ...overrides,
  };
}

/** Create a callable export fixture with optional field overrides. */
export function mockExportDescriptor(overrides: Partial<TCallableExportDescriptor> = {}): TCallableExportDescriptor {
  return {
    kind: "callable",
    name: "play_sound",
    description: null,
    parameters: [{ name: "actor", typing: "game_object", description: null, isOptional: false }],
    returns: { typing: "void", description: null },
    source: { path: "xr_effects.ts", line: 42, column: 2, endLine: 45 },
    ...overrides,
  };
}

/** Create export declaration fixtures with optional additional declarations. */
export function mockExportsDeclarations(overrides: Array<ExportDescriptor> = []): Array<ExportDescriptor> {
  return [
    mockExportDescriptor({
      source: { path: "xr_conditions.ts", line: 1, column: 1, endLine: 4 },
      name: "xr_conditions.is_wounded",
    }),
    mockExportDescriptor({ source: { path: "dialogs.ts", line: 1, column: 1, endLine: 4 }, name: "dialogs.is_friend" }),
    mockExportDescriptor({ source: { path: "dialogs.ts", line: 1, column: 1, endLine: 4 }, name: "dialogs.has_item" }),
    mockExportDescriptor({ name: "xr_effects.play_sound" }),
    ...overrides,
  ];
}

/** Create an exports project fixture with optional field overrides. */
export function mockExportsProject(overrides: Partial<ExportsProject> = {}): ExportsProject {
  return {
    root: "C:\\projects\\xrf",
    declarations: mockExportsDeclarations(),
    ...overrides,
  };
}

/** Create a translations project fixture with optional field overrides. */
export function mockTranslationsProject(overrides: Partial<ITranslationsProjectJson> = {}): ITranslationsProjectJson {
  return {
    "st_dialogs.json": {
      dialog_greeting: "Hello, stalker",
    },
    "st_items.json": {
      wpn_ak74: "AK-74",
      wpn_ak74_descr: "Assault rifle",
    },
    ...overrides,
  };
}

/** Create equipment descriptor fixtures. */
export function mockEquipmentDescriptors(): Array<IEquipmentSectionDescriptor> {
  return [
    { section: "wpn_ak74", w: 2, h: 1, x: 0, y: 0 },
    { section: "wpn_pm", w: 1, h: 1, x: 2, y: 0 },
  ];
}

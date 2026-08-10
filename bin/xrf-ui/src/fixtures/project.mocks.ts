import { IArchiveDescriptor, IArchiveFileReplicationDescriptor, IArchivesProject } from "@/lib/archive";
import { IExportDescriptor, TExportsDeclarations } from "@/lib/exports";
import { IEquipmentSectionDescriptor } from "@/lib/icons";
import { ITranslationsProjectJson } from "@/lib/translations";

export function mockArchiveFile(
  overrides: Partial<IArchiveFileReplicationDescriptor> = {}
): IArchiveFileReplicationDescriptor {
  return {
    crc: 123456,
    destination: "gamedata\\config\\system.ltx",
    name: "system.ltx",
    offset: 0,
    sizeCompressed: 512,
    sizeReal: 2048,
    source: "db\\db0",
    ...overrides,
  };
}

export function mockArchiveDescriptor(overrides: Partial<IArchiveDescriptor> = {}): IArchiveDescriptor {
  return {
    files: {},
    outputRootPath: "unpacked",
    path: "db\\db0",
    ...overrides,
  };
}

export function mockArchivesProject(overrides: Partial<IArchivesProject> = {}): IArchivesProject {
  return {
    archives: [mockArchiveDescriptor(), mockArchiveDescriptor({ path: "db\\db1" })],
    files: {
      "config\\system.ltx": mockArchiveFile(),
      "config\\weapons\\wpn_ak74.ltx": mockArchiveFile({
        name: "wpn_ak74.ltx",
        destination: "gamedata\\config\\weapons\\wpn_ak74.ltx",
      }),
      "meshes\\dynamics\\weapons\\wpn_ak74.ogf": mockArchiveFile({
        name: "wpn_ak74.ogf",
        destination: "gamedata\\meshes\\dynamics\\weapons\\wpn_ak74.ogf",
      }),
    },
    ...overrides,
  };
}

export function mockExportDescriptor(overrides: Partial<IExportDescriptor> = {}): IExportDescriptor {
  return {
    filename: "xr_effects.ts",
    name: "play_sound",
    comment: null,
    parameters: [{ name: "actor", typing: "game_object", comment: null }],
    typing: null,
    line: 42,
    col: 2,
    ...overrides,
  };
}

export function mockExportsDeclarations(overrides: TExportsDeclarations = []): TExportsDeclarations {
  return [
    mockExportDescriptor({ filename: "xr_conditions.ts", name: "xr_conditions.is_wounded" }),
    mockExportDescriptor({ filename: "dialogs.ts", name: "dialogs.is_friend" }),
    mockExportDescriptor({ filename: "dialogs.ts", name: "dialogs.has_item" }),
    mockExportDescriptor({ name: "xr_effects.play_sound" }),
    ...overrides,
  ];
}

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

export function mockEquipmentDescriptors(): Array<IEquipmentSectionDescriptor> {
  return [
    { section: "wpn_ak74", w: 2, h: 1, x: 0, y: 0 },
    { section: "wpn_pm", w: 1, h: 1, x: 2, y: 0 },
  ];
}

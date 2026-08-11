import { IArchiveDescriptor, IArchiveFileDescriptor, IArchiveReadPolicy, IArchivesProject } from "@/lib/archive";

export function mockArchiveReadPolicy(overrides: Partial<IArchiveReadPolicy> = {}): IArchiveReadPolicy {
  return {
    extensions: ["ltx", "script", "ps", "ds", "h", "hs", "s", "vs", "cmd", "xml"],
    maximumSize: 10 * 1024 * 1024,
    supportsCompressedFiles: false,
    ...overrides,
  };
}

export function mockArchiveFileDescriptor(
  overrides: Partial<IArchiveFileDescriptor> = {}
): IArchiveFileDescriptor {
  return {
    crc: 0x12345678,
    destination: "gamedata",
    extension: "ltx",
    name: "configs\\system.ltx",
    offset: 4096,
    sizeCompressed: 2048,
    sizeReal: 2048,
    source: "C:\\game\\database\\configs.db0",
    ...overrides,
  };
}

export function mockArchivesProject(files?: Array<IArchiveFileDescriptor>): IArchivesProject {
  const descriptors: Array<IArchiveFileDescriptor> = files ?? [
    mockArchiveFileDescriptor(),
    mockArchiveFileDescriptor({
      extension: "script",
      name: "scripts\\actor.script",
      sizeReal: 1024,
      sizeCompressed: 1024,
    }),
  ];
  const archive: IArchiveDescriptor = {
    files: {},
    outputRootPath: "gamedata",
    path: "C:\\game\\database\\configs.db0",
  };

  return {
    archives: [archive],
    files: Object.fromEntries(descriptors.map((descriptor) => [descriptor.name, descriptor])),
    readPolicy: mockArchiveReadPolicy(),
    root: "C:\\game\\database",
    sizeReal: descriptors.reduce((total: number, descriptor) => total + descriptor.sizeReal, 0),
  };
}

import { IArchiveDescriptor, IArchiveFileReplicationDescriptor, IArchivesProject } from "@/lib/archive";

export function mockArchiveFileDescriptor(
  overrides: Partial<IArchiveFileReplicationDescriptor> = {}
): IArchiveFileReplicationDescriptor {
  return {
    crc: 0x12345678,
    destination: "gamedata",
    name: "configs\\system.ltx",
    offset: 4096,
    sizeCompressed: 2048,
    sizeReal: 2048,
    source: "C:\\game\\database\\configs.db0",
    ...overrides,
  };
}

export function mockArchivesProject(files?: Array<IArchiveFileReplicationDescriptor>): IArchivesProject {
  const descriptors: Array<IArchiveFileReplicationDescriptor> = files ?? [
    mockArchiveFileDescriptor(),
    mockArchiveFileDescriptor({ name: "scripts\\actor.script", sizeReal: 1024, sizeCompressed: 1024 }),
  ];
  const archive: IArchiveDescriptor = {
    files: {},
    outputRootPath: "gamedata",
    path: "C:\\game\\database\\configs.db0",
  };

  return {
    archives: [archive],
    files: Object.fromEntries(descriptors.map((descriptor) => [descriptor.name, descriptor])),
  };
}

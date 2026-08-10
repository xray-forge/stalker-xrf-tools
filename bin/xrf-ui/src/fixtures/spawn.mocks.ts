import { IAlifeObjectBase, IPatrol, ISpawnFile, IVector3d } from "@/lib/spawn_file";

export function mockVector3d(overrides: Partial<IVector3d> = {}): IVector3d {
  return { x: 0, y: 0, z: 0, ...overrides };
}

export function mockAlifeObject(overrides: Partial<IAlifeObjectBase> = {}): IAlifeObjectBase {
  return {
    clientDataSize: 0,
    clsid: "AI_STL_S",
    direction: mockVector3d(),
    gameType: 1,
    id: 1,
    inherited: { type: "cse_alife_human_stalker" },
    name: "esc_smart_stalker_1",
    netAction: 1,
    parentId: 65535,
    phantomId: 65535,
    position: mockVector3d({ x: 12.5, y: 1.25, z: -30 }),
    respawnTime: 0,
    scriptFlags: 0,
    scriptGameId: 0,
    scriptRp: 0,
    scriptVersion: 12,
    section: "stalker_novice",
    spawnId: 1,
    updateData: [],
    version: 124,
    ...overrides,
  };
}

export function mockPatrol(overrides: Partial<IPatrol> = {}): IPatrol {
  return {
    name: "esc_walker_walk",
    points: [
      {
        name: "wp00|a=patrol",
        position: mockVector3d({ x: 1, y: 0, z: 1 }),
        flags: 1,
        levelVertexId: 100,
        gameVertexId: 10,
      },
      {
        name: "wp01",
        position: mockVector3d({ x: 4, y: 0, z: 2 }),
        flags: 1,
        levelVertexId: 101,
        gameVertexId: 10,
      },
    ],
    links: [{ index: 0, links: [[1, 1]] }],
    ...overrides,
  };
}

/**
 * A small but structurally complete spawn file.
 *
 * Counts in the header deliberately match the collections below, because editors report them and a
 * fixture that disagrees with itself makes those assertions meaningless.
 */
export function mockSpawnFile(overrides: Partial<ISpawnFile> = {}): ISpawnFile {
  return {
    header: {
      version: 124,
      guid: "8f1a1b2c-0000-4000-8000-000000000001",
      graphGuid: "8f1a1b2c-0000-4000-8000-000000000002",
      objectsCount: 2,
      levelsCount: 1,
    },
    alifeSpawn: {
      objects: [mockAlifeObject(), mockAlifeObject({ id: 2, name: "esc_smart_stalker_2", spawnId: 2 })],
    },
    artefactSpawn: {
      nodes: [{ position: mockVector3d({ x: 5, y: 0, z: 5 }), levelVertexId: 200, distance: 1.5 }],
    },
    patrols: { patrols: [mockPatrol()] },
    graphs: {
      header: {
        guid: "8f1a1b2c-0000-4000-8000-000000000002",
        edgesCount: 1,
        levelsCount: 1,
        pointsCount: 1,
        version: 10,
        verticesCount: 1,
      },
      levels: [
        {
          id: 1,
          guid: "8f1a1b2c-0000-4000-8000-000000000003",
          name: "l01_escape",
          offset: mockVector3d(),
          section: "level_escape",
        },
      ],
      crossTables: [
        {
          version: 16,
          gameGuid: "8f1a1b2c-0000-4000-8000-000000000002",
          levelGuid: "8f1a1b2c-0000-4000-8000-000000000003",
          nodesCount: 1,
          vertexCount: 1,
        },
      ],
      edges: [{ distance: 12.5, gameVertexId: 1 }],
      points: [{ distance: 3.25, levelVertexId: 100 }],
      vertices: [
        {
          edgesCount: 1,
          edgesOffset: 0,
          levelId: 1,
          levelPointCount: 1,
          levelPointOffset: 0,
          levelVertexId: 100,
          vertexType: [0, 0, 0, 0],
          gamePoint: mockVector3d(),
          levelPoint: mockVector3d(),
        },
      ],
    },
    ...overrides,
  };
}

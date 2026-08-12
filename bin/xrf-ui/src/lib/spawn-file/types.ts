import { SpawnGraphsChunk, SpawnHeaderChunk, Vector3d } from "@/lib/rust-sdk/xray-db";

export interface ISpawnFile {
  header: SpawnHeaderChunk;
  alifeSpawn: ISpawnFileAlifeSpawnsChunk;
  artefactSpawn: ISpawnFileArtefactSpawnsChunk;
  patrols: ISpawnFilePatrolsChunk;
  graphs: SpawnGraphsChunk;
}

export interface ISpawnFileAlifeSpawnsChunk {
  objects: Array<IAlifeObjectBase>;
}

export interface ISpawnFileArtefactSpawnsChunk {
  nodes: Array<IArtefactSpawnNode>;
}

export interface ISpawnFilePatrolsChunk {
  patrols: Array<IPatrol>;
}

export interface IArtefactSpawnNode {
  position: Vector3d;
  levelVertexId: number;
  distance: number;
}

export interface IPatrol {
  name: string;
  points: Array<IPatrolPoint>;
  links: Array<IPatrolLink>;
}

export interface IPatrolPoint {
  name: string;
  position: Vector3d;
  flags: number;
  levelVertexId: number;
  gameVertexId: number;
}

export interface IPatrolLink {
  index: number;
  links: Array<[number, number]>;
}

export interface IAlifeObjectBase {
  clientDataSize: number;
  clsid: string;
  direction: Vector3d;
  gameType: number;
  id: number;
  inherited: {
    [index: string]: unknown;
    type: string;
  };
  name: string;
  netAction: number;
  parentId: number;
  phantomId: number;
  position: Vector3d;
  respawnTime: number;
  scriptFlags: number;
  scriptGameId: number;
  scriptRp: number;
  scriptVersion: number;
  section: string;
  spawnId: number;
  updateData: Array<unknown>;
  version: number;
}

export interface ISpawnFile {
  header: ISpawnFileHeaderChunk;
  alifeSpawn: ISpawnFileAlifeSpawnsChunk;
  artefactSpawn: ISpawnFileArtefactSpawnsChunk;
  patrols: ISpawnFilePatrolsChunk;
  graphs: ISpawnFileGraphsChunk;
}

export interface ISpawnFileHeaderChunk {
  version: number;
  guid: string;
  graphGuid: string;
  objectsCount: number;
  levelsCount: number;
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

export interface ISpawnFileGraphsChunk {
  header: IGraphHeader;
  levels: Array<IGraphLevel>;
  crossTables: Array<IGraphCrossTable>;
  edges: Array<IGraphEdge>;
  points: Array<IGraphPoint>;
  vertices: Array<IGraphVertex>;
}

export interface IArtefactSpawnNode {
  position: IVector3d;
  levelVertexId: number;
  distance: number;
}

export interface IGraphVertex {
  edgesCount: number;
  edgesOffset: number;
  levelId: number;
  levelPointCount: number;
  levelPointOffset: number;
  levelVertexId: number;
  vertexType: [number, number, number, number];
  gamePoint: IVector3d;
  levelPoint: IVector3d;
}

export interface IGraphLevel {
  id: number;
  guid: string;
  name: string;
  offset: IVector3d;
  section: string;
}

export interface IGraphPoint {
  distance: number;
  levelVertexId: number;
}

export interface IGraphHeader {
  guid: string;
  edgesCount: number;
  levelsCount: number;
  pointsCount: number;
  version: number;
  verticesCount: number;
}

export interface IGraphCrossTable {
  version: number;
  gameGuid: string;
  levelGuid: string;
  nodesCount: number;
  vertexCount: number;
}

export interface IGraphEdge {
  distance: number;
  gameVertexId: number;
}

export interface IPatrol {
  name: string;
  points: Array<IPatrolPoint>;
  links: Array<IPatrolLink>;
}

export interface IPatrolPoint {
  name: string;
  position: IVector3d;
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
  direction: IVector3d;
  gameType: number;
  id: number;
  index: number;
  inherited: unknown;
  name: string;
  netAction: number;
  parentId: number;
  phantomId: number;
  position: IVector3d;
  respawnTime: number;
  scriptFlags: number;
  scriptGameId: number;
  scriptRp: number;
  scriptVersion: number;
  section: string;
  spawnId: number;
  updateData: unknown;
  version: number;
}

export interface IVector3d {
  x: number;
  y: number;
  z: number;
}

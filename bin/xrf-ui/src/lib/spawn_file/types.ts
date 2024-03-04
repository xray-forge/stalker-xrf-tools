export interface ISpawnFile {
  header: ISpawnFileHeaderChunk;
  alife_spawn: ISpawnFileAlifeSpawnsChunk;
  artefact_spawn: ISpawnFileArtefactSpawnsChunk;
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
  cross_tables: Array<IGraphCrossTable>;
  edges: Array<IGraphEdge>;
  points: Array<IGraphPoint>;
  vertices: Array<IGraphVertex>;
}

export interface IArtefactSpawnNode {
  position: IVector3d;
  level_vertex_id: number;
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
  client_data_size: number;
  clsid: string;
  direction: IVector3d;
  game_type: number;
  id: number;
  index: number;
  inherited: unknown;
  name: string;
  net_action: number;
  parent_id: number;
  phantom_id: number;
  position: IVector3d;
  respawn_time: number;
  script_flags: number;
  script_game_id: number;
  script_rp: number;
  script_version: number;
  section: string;
  spawn_id: number;
  update_data: unknown;
  version: number;
}

export interface IVector3d {
  x: number;
  y: number;
  z: number;
}

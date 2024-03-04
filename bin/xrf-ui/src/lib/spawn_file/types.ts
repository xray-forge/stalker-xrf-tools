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
  graph_guid: string;
  objects_count: number;
  level_count: number;
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
  cross_tables: Array<ICrossTable>;
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
  edge_count: number;
  edge_offset: number;
  level_id: number;
  level_point_count: number;
  level_point_offset: number;
  level_vertex_id: number;
  vertex_type: [number, number, number, number];
  game_point: IVector3d;
  level_point: IVector3d;
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
  level_vertex_id: number;
}

export interface IGraphHeader {
  guid: string;
  edges_count: number;
  level_count: number;
  point_count: number;
  version: number;
  vertex_count: number;
}

export interface ICrossTable {
  version: number;
  game_guid: string;
  level_guid: string;
  nodes_count: number;
  vertex_count: number;
}

export interface IGraphEdge {
  distance: number;
  game_vertex_id: number;
}

export interface IPatrol {
  name: string;
  points: Array<IPatrolPoint>;
  links: Array<IPatrolLink>;
}

export interface IVector3d {
  x: number;
  y: number;
  z: number;
}

export interface IPatrolPoint {
  name: string;
  position: IVector3d;
  flags: number;
  level_vertex_id: number;
  game_vertex_id: number;
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

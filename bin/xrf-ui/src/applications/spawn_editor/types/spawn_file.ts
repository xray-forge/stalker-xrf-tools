export interface ISpawnFileHeader {
  version: number;
  guid: string;
  graph_guid: string;
  objects_count: number;
  level_count: number;
}

export interface ISpawnFilePatrols {
  patrols: Array<IPatrol>;
}

export interface ISpawnFileArtefactSpawns {
  nodes: Array<IArtefactSpawnNode>;
}

export interface IArtefactSpawnNode {
  position: IVector3d;
  level_vertex_id: number;
  distance: number;
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

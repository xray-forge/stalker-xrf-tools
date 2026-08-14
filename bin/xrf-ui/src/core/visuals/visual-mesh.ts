/**
 * Renderer facing mesh data.
 *
 * Flat typed arrays rather than an array of vertex objects, because that is what the GPU consumes and
 * what the rust side is expected to hand over once `OgfVertex` data is packed for rendering. Keeping
 * this contract in place from the start means swapping the stub below for real geometry is a change of
 * data source and nothing else.
 */
export interface IVisualMeshData {
  positions: Float32Array;
  normals: Float32Array;
  uvs: Float32Array;
  indices: Uint16Array;
}

/**
 * One face of the stub cube, corners listed counter clockwise as seen from outside.
 */
interface IStubMeshFace {
  normal: Array<number>;
  corners: Array<Array<number>>;
}

const STUB_MESH_HALF_SIZE: number = 0.5;

const STUB_MESH_FACE_UVS: Array<Array<number>> = [
  [0, 0],
  [1, 0],
  [1, 1],
  [0, 1],
];

const STUB_MESH_FACES: Array<IStubMeshFace> = [
  {
    normal: [1, 0, 0],
    corners: [
      [1, -1, 1],
      [1, -1, -1],
      [1, 1, -1],
      [1, 1, 1],
    ],
  },
  {
    normal: [-1, 0, 0],
    corners: [
      [-1, -1, -1],
      [-1, -1, 1],
      [-1, 1, 1],
      [-1, 1, -1],
    ],
  },
  {
    normal: [0, 1, 0],
    corners: [
      [-1, 1, 1],
      [1, 1, 1],
      [1, 1, -1],
      [-1, 1, -1],
    ],
  },
  {
    normal: [0, -1, 0],
    corners: [
      [-1, -1, -1],
      [1, -1, -1],
      [1, -1, 1],
      [-1, -1, 1],
    ],
  },
  {
    normal: [0, 0, 1],
    corners: [
      [-1, -1, 1],
      [1, -1, 1],
      [1, 1, 1],
      [-1, 1, 1],
    ],
  },
  {
    normal: [0, 0, -1],
    corners: [
      [1, -1, -1],
      [-1, -1, -1],
      [-1, 1, -1],
      [1, 1, -1],
    ],
  },
];

/**
 * Build a hardcoded cube in the same layout real visuals will arrive in.
 *
 * Written out as explicit attribute buffers instead of using `BoxGeometry`, so the prototype exercises
 * the path an ogf visual will take rather than a shortcut that proves nothing.
 *
 * @returns Cube mesh data in the renderer's attribute-buffer layout.
 */
export function createStubVisualMeshData(): IVisualMeshData {
  const vertexCount: number = STUB_MESH_FACES.length * 4;
  const positions: Float32Array = new Float32Array(vertexCount * 3);
  const normals: Float32Array = new Float32Array(vertexCount * 3);
  const uvs: Float32Array = new Float32Array(vertexCount * 2);
  const indices: Uint16Array = new Uint16Array(STUB_MESH_FACES.length * 6);

  STUB_MESH_FACES.forEach((face, faceIndex) => {
    const base: number = faceIndex * 4;

    face.corners.forEach((corner, cornerIndex) => {
      const vertex: number = base + cornerIndex;

      positions.set(
        corner.map((it) => it * STUB_MESH_HALF_SIZE),
        vertex * 3
      );
      normals.set(face.normal, vertex * 3);
      uvs.set(STUB_MESH_FACE_UVS[cornerIndex], vertex * 2);
    });

    indices.set([base, base + 1, base + 2, base, base + 2, base + 3], faceIndex * 6);
  });

  return { positions, normals, uvs, indices };
}

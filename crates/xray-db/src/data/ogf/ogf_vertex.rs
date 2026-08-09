use byteorder::ByteOrder;
use serde::{Deserialize, Serialize};

use crate::data::generic::vector_3d::Vector3d;

/// One bone a vertex is skinned to, and how strongly.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfVertexLink {
  pub bone: u16,
  pub weight: f32,
}

/// One vertex of an OGF visual, with its skinning links resolved.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OgfVertex {
  pub position: Vector3d,
  pub normal: Vector3d,
  pub tangent: Vector3d,
  pub binormal: Vector3d,
  pub texture_u: f32,
  pub texture_v: f32,
  pub links: Vec<OgfVertexLink>,
}

impl OgfVertex {
  /// Bytes of geometry every format shares: four vectors and a uv pair.
  pub const GEOMETRY_SIZE: usize = 3 * 4 * 4 + 2 * 4;

  /// Read one vertex out of an already sized slice.
  ///
  /// A single link stores its bone as a `u32` after the geometry; two or more store their bones as
  /// `u16` before it, followed by one fewer weight than there are bones. The final weight is not
  /// stored because the set sums to one, so it is reconstructed here and every returned vertex carries
  /// a weight for each of its links.
  pub fn read_from_slice<T: ByteOrder>(vertex: &[u8], links_count: usize) -> Self {
    debug_assert!(links_count >= 1, "a vertex is linked to at least one bone");

    if links_count == 1 {
      let geometry: Self = Self::read_geometry::<T>(&vertex[..Self::GEOMETRY_SIZE], Vec::new());

      return Self {
        links: vec![OgfVertexLink {
          bone: T::read_u32(&vertex[Self::GEOMETRY_SIZE..Self::GEOMETRY_SIZE + 4]) as u16,
          weight: 1.0,
        }],
        ..geometry
      };
    }

    let bones_size: usize = links_count * 2;
    let weights_size: usize = (links_count - 1) * 4;

    let mut links: Vec<OgfVertexLink> = Vec::with_capacity(links_count);
    let mut remaining_weight: f32 = 1.0;

    for index in 0..links_count {
      let bone: u16 = T::read_u16(&vertex[index * 2..index * 2 + 2]);

      // The last link takes whatever weight the stored ones did not claim.
      let weight: f32 = if index + 1 == links_count {
        remaining_weight
      } else {
        let offset: usize = bones_size + index * 4;
        let weight: f32 = T::read_f32(&vertex[offset..offset + 4]);

        remaining_weight -= weight;

        weight
      };

      links.push(OgfVertexLink { bone, weight });
    }

    let geometry_offset: usize = bones_size + weights_size;

    Self::read_geometry::<T>(&vertex[geometry_offset..geometry_offset + Self::GEOMETRY_SIZE], links)
  }

  fn read_geometry<T: ByteOrder>(geometry: &[u8], links: Vec<OgfVertexLink>) -> Self {
    let vector = |offset: usize| Vector3d {
      x: T::read_f32(&geometry[offset..offset + 4]),
      y: T::read_f32(&geometry[offset + 4..offset + 8]),
      z: T::read_f32(&geometry[offset + 8..offset + 12]),
    };

    Self {
      position: vector(0),
      normal: vector(12),
      tangent: vector(24),
      binormal: vector(36),
      texture_u: T::read_f32(&geometry[48..52]),
      texture_v: T::read_f32(&geometry[52..56]),
      links,
    }
  }
}

#[cfg(test)]
mod tests {
  use xray_chunk::XRayByteOrder;

  use super::OgfVertex;

  /// Geometry block shared by every format: position, normal, tangent, binormal, then a uv pair.
  fn geometry_bytes() -> Vec<u8> {
    let floats: [f32; 14] = [
      1.0, 2.0, 3.0, // position
      0.0, 1.0, 0.0, // normal
      1.0, 0.0, 0.0, // tangent
      0.0, 0.0, 1.0, // binormal
      0.25, 0.75, // uv
    ];

    floats.iter().flat_map(|it| it.to_le_bytes()).collect()
  }

  fn assert_geometry(vertex: &OgfVertex) {
    assert_eq!(
      (vertex.position.x, vertex.position.y, vertex.position.z),
      (1.0, 2.0, 3.0)
    );
    assert_eq!((vertex.normal.x, vertex.normal.y, vertex.normal.z), (0.0, 1.0, 0.0));
    assert_eq!((vertex.tangent.x, vertex.tangent.y, vertex.tangent.z), (1.0, 0.0, 0.0));
    assert_eq!(
      (vertex.binormal.x, vertex.binormal.y, vertex.binormal.z),
      (0.0, 0.0, 1.0)
    );
    assert_eq!((vertex.texture_u, vertex.texture_v), (0.25, 0.75));
  }

  #[test]
  fn reads_a_single_link_vertex() {
    // One link stores its bone as a u32 after the geometry, and carries no stored weight.
    let mut bytes: Vec<u8> = geometry_bytes();
    bytes.extend_from_slice(&7u32.to_le_bytes());

    let vertex: OgfVertex = OgfVertex::read_from_slice::<XRayByteOrder>(&bytes, 1);

    assert_geometry(&vertex);
    assert_eq!(vertex.links.len(), 1);
    assert_eq!(vertex.links[0].bone, 7);
    assert_eq!(
      vertex.links[0].weight, 1.0,
      "Expect a lone link to own the whole weight"
    );
  }

  #[test]
  fn reads_a_two_link_vertex_and_reconstructs_the_last_weight() {
    // Two links store both bones first, then one weight; the second is implied.
    let mut bytes: Vec<u8> = Vec::new();
    bytes.extend_from_slice(&3u16.to_le_bytes());
    bytes.extend_from_slice(&9u16.to_le_bytes());
    bytes.extend_from_slice(&0.25f32.to_le_bytes());
    bytes.extend(geometry_bytes());

    let vertex: OgfVertex = OgfVertex::read_from_slice::<XRayByteOrder>(&bytes, 2);

    assert_geometry(&vertex);
    assert_eq!(vertex.links.len(), 2);
    assert_eq!((vertex.links[0].bone, vertex.links[0].weight), (3, 0.25));
    assert_eq!(
      (vertex.links[1].bone, vertex.links[1].weight),
      (9, 0.75),
      "Expect the unstored final weight to complete the set"
    );
  }

  #[test]
  fn reads_a_four_link_vertex_with_weights_summing_to_one() {
    let mut bytes: Vec<u8> = Vec::new();
    for bone in [1u16, 2, 3, 4] {
      bytes.extend_from_slice(&bone.to_le_bytes());
    }
    for weight in [0.1f32, 0.2, 0.3] {
      bytes.extend_from_slice(&weight.to_le_bytes());
    }
    bytes.extend(geometry_bytes());

    let vertex: OgfVertex = OgfVertex::read_from_slice::<XRayByteOrder>(&bytes, 4);

    assert_geometry(&vertex);
    assert_eq!(
      vertex.links.iter().map(|it| it.bone).collect::<Vec<u16>>(),
      vec![1, 2, 3, 4]
    );
    assert!(
      (vertex.links.iter().map(|it| it.weight).sum::<f32>() - 1.0).abs() < 1e-6,
      "Expect weights to sum to one, got {:?}",
      vertex.links.iter().map(|it| it.weight).collect::<Vec<f32>>()
    );
    assert!((vertex.links[3].weight - 0.4).abs() < 1e-6);
  }
}

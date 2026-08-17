use xrf_db::{OgfFile, OgfGeometry, OgfSlideWindow, OgfVertex, Vector3d};

use crate::data::visual_bounds::VisualBounds;
use crate::data::visual_description::{VisualBone, VisualDescription};
use crate::data::visual_model_type::VisualModelType;
use crate::data::visual_section::{VisualDrawRange, VisualSlideWindow};
use crate::data::visual_submesh::{VisualGeometry, VisualSkipCause, VisualSubmesh, VisualSubmeshContent};
use crate::pack::visual_buffer_builder::VisualBufferBuilder;
use crate::pack::visual_conversion::{convert_declared_bounds, convert_texture_coordinates, convert_vector};
use crate::pack::visual_package::VisualPackage;

/// A submesh that produced no geometry, as the packer's internal early return.
///
/// Becomes [`VisualSubmeshContent::Skipped`] verbatim, so every reason below is one a consumer reads.
struct VisualSkip {
  cause: VisualSkipCause,
  reason: String,
}

impl VisualSkip {
  /// Geometry the packer cannot read, which is a gap in coverage rather than a broken file.
  fn unsupported(reason: impl Into<String>) -> Self {
    Self {
      cause: VisualSkipCause::Unsupported,
      reason: reason.into(),
    }
  }

  /// Geometry that contradicts itself, which no amount of added coverage would fix.
  fn malformed(reason: impl Into<String>) -> Self {
    Self {
      cause: VisualSkipCause::Malformed,
      reason: reason.into(),
    }
  }
}

/// Flattens a parsed OGF visual into renderer ready buffers.
///
/// Packing never fails. A child that carries nothing drawable becomes a submesh holding the reason it
/// was skipped, so one broken piece neither hides the rest of a model nor turns into an error the
/// caller has to interpret.
pub struct VisualPacker {}

impl VisualPacker {
  pub fn pack(file: &OgfFile) -> VisualPackage {
    let mut builder: VisualBufferBuilder = VisualBufferBuilder::new();

    let submeshes: Vec<VisualSubmesh> = Self::submesh_sources(file)
      .into_iter()
      .enumerate()
      .map(|(index, source)| Self::pack_submesh(&mut builder, index as u32, source))
      .collect();

    let computed_bounds: Option<VisualBounds> = submeshes
      .iter()
      .filter_map(|submesh| submesh.geometry())
      .map(|geometry| geometry.bounds.clone())
      .reduce(VisualBounds::merge);

    let description: VisualDescription = VisualDescription {
      version: file.header.version,
      model_type: file.header.model_type,
      model_type_label: VisualModelType::label(file.header.model_type),
      shader_id: file.header.shader_id,
      source_file: file
        .description
        .as_ref()
        .map(|it| it.source_file.clone())
        .filter(|it| !it.is_empty()),
      declared_bounds: convert_declared_bounds(&file.header.bounding_box, &file.header.bounding_sphere),
      computed_bounds,
      submeshes,
      bones: file
        .bones
        .as_ref()
        .map(|it| {
          it.bones
            .iter()
            .map(|bone| VisualBone {
              name: bone.name.clone(),
              parent: bone.parent.clone(),
            })
            .collect()
        })
        .unwrap_or_default(),
      motion_refs: file
        .kinematics
        .as_ref()
        .map(|it| it.motion_refs.clone())
        .unwrap_or_default(),
      embedded_motions: file
        .motions
        .as_ref()
        .map(|it| it.motions.iter().map(|motion| motion.name.clone()).collect())
        .unwrap_or_default(),
      buffer_length: builder.length(),
    };

    VisualPackage {
      description,
      buffer: builder.into_buffer(),
    }
  }

  /// Drawable pieces of a visual, in the order the file stores them.
  ///
  /// A skeleton keeps its geometry on children and carries none itself; a single level visual is its
  /// own only piece. Submesh order is the child order, because a texture or shader reference is only
  /// meaningful against the child it came from.
  fn submesh_sources(file: &OgfFile) -> Vec<&OgfFile> {
    match file.children.as_ref().map(|it| it.nested.as_slice()) {
      Some(nested) if !nested.is_empty() => nested.iter().collect(),
      _ => vec![file],
    }
  }

  fn pack_submesh(builder: &mut VisualBufferBuilder, index: u32, source: &OgfFile) -> VisualSubmesh {
    let model_type: u8 = source.header.model_type;

    VisualSubmesh {
      index,
      model_type,
      model_type_label: VisualModelType::label(model_type),
      texture_name: source.texture.as_ref().map(|it| it.texture_name.clone()),
      shader_name: source.texture.as_ref().map(|it| it.shader_name.clone()),
      content: match Self::pack_geometry(builder, source) {
        Ok(geometry) => VisualSubmeshContent::Packed { geometry },
        Err(skip) => VisualSubmeshContent::Skipped {
          cause: skip.cause,
          reason: skip.reason,
        },
      },
    }
  }

  /// Convert and append one submesh's attributes, or say why it has none.
  ///
  /// The error type is the reason a consumer displays, not a failure: every early return here ends up
  /// beside the submesh in the description.
  fn pack_geometry(builder: &mut VisualBufferBuilder, source: &OgfFile) -> Result<VisualGeometry, VisualSkip> {
    // Geometry can live in a shared vertex or index container outside the file, in which case the
    // chunk is legitimately absent rather than missing, so none of these are malformed files.
    let geometry: &OgfGeometry = source
      .geometry
      .as_ref()
      .ok_or_else(|| VisualSkip::unsupported("Carries no geometry chunk"))?;

    let vertices: &Vec<OgfVertex> = geometry.vertices.as_ref().ok_or_else(|| match geometry.vertex_format {
      Some(format) => VisualSkip::unsupported(format!("Vertex format {format:#010x} has no known layout")),
      None => VisualSkip::unsupported("Carries no vertex chunk"),
    })?;

    if vertices.is_empty() {
      return Err(VisualSkip::malformed("Vertex chunk is empty"));
    }

    let indices: &Vec<u16> = geometry
      .indices
      .as_ref()
      .ok_or_else(|| VisualSkip::unsupported("Carries no index chunk"))?;

    if indices.is_empty() {
      return Err(VisualSkip::malformed("Index chunk is empty"));
    }

    if !indices.len().is_multiple_of(3) {
      return Err(VisualSkip::malformed(format!(
        "Index count {} is not a whole number of triangles",
        indices.len()
      )));
    }

    let windows: Vec<VisualSlideWindow> = source
      .swi_data
      .as_ref()
      .map(|swi| swi.windows.iter().map(Self::convert_window).collect())
      .unwrap_or_default();

    let draw_range: VisualDrawRange = Self::resolve_draw_range(source.header.model_type, &windows, indices.len())?;

    Self::assert_drawn_indices_in_range(indices, draw_range, vertices.len())?;

    let positions: Vec<Vector3d> = vertices.iter().map(|it| convert_vector(&it.position)).collect();
    let drawn_start: usize = draw_range.start as usize;
    let bounds: VisualBounds = VisualBounds::from_indexed_positions(
      &positions,
      &indices[drawn_start..drawn_start + draw_range.count as usize],
    )
    .ok_or_else(|| VisualSkip::malformed("Drawn range reaches no vertex"))?;

    let flat_positions: Vec<f32> = positions.iter().flat_map(|it| [it.x, it.y, it.z]).collect();
    let flat_normals: Vec<f32> = vertices
      .iter()
      .flat_map(|it| {
        let normal: Vector3d = convert_vector(&it.normal);

        [normal.x, normal.y, normal.z]
      })
      .collect();
    let flat_uvs: Vec<f32> = vertices
      .iter()
      .flat_map(|it| {
        let (u, v) = convert_texture_coordinates(it.texture_u, it.texture_v);

        [u, v]
      })
      .collect();

    let mut wound_indices: Vec<u16> = indices.clone();

    reverse_triangle_winding(&mut wound_indices);

    Ok(VisualGeometry {
      vertex_count: vertices.len() as u32,
      index_count: wound_indices.len() as u32,
      positions: builder.push_f32_section(&flat_positions),
      normals: builder.push_f32_section(&flat_normals),
      uvs: builder.push_f32_section(&flat_uvs),
      indices: builder.push_u16_section(&wound_indices),
      draw_range,
      windows,
      bounds,
    })
  }

  /// The index range that draws a submesh at full detail.
  ///
  /// Static geometry draws its whole buffer. Progressive geometry stores every detail level in that
  /// same buffer with the finest one at level zero, which `FSkinned.cpp:419` selects when it wants
  /// full geometry, so drawing all of it would stack the coarse shells over the fine mesh.
  fn resolve_draw_range(
    model_type: u8,
    windows: &[VisualSlideWindow],
    index_count: usize,
  ) -> Result<VisualDrawRange, VisualSkip> {
    let is_progressive: bool = VisualModelType::from_raw(model_type).is_some_and(VisualModelType::is_progressive);

    if !is_progressive {
      return Ok(VisualDrawRange {
        start: 0,
        count: index_count as u32,
      });
    }

    // Falling back to the whole buffer would draw every detail level at once, which reads as a fatter
    // model rather than as an error, so a progressive submesh without its table is refused instead.
    let window: &VisualSlideWindow = windows.first().ok_or_else(|| {
      VisualSkip::malformed("Progressive geometry carries no detail table, so its full detail range is unknown")
    })?;

    let count: u32 = window.triangle_count * 3;

    if window.offset as u64 + count as u64 > index_count as u64 {
      return Err(VisualSkip::malformed(format!(
        "Detail level 0 draws {} indices from offset {}, past the {} the index chunk holds",
        count, window.offset, index_count
      )));
    }

    Ok(VisualDrawRange {
      start: window.offset,
      count,
    })
  }

  /// Reject a drawn range that addresses a vertex the submesh does not have.
  ///
  /// Only the drawn range is checked. Coarser detail levels are shipped but never dereferenced, so a
  /// consumer that starts drawing one must range check it then rather than trusting this.
  fn assert_drawn_indices_in_range(
    indices: &[u16],
    draw_range: VisualDrawRange,
    vertex_count: usize,
  ) -> Result<(), VisualSkip> {
    let start: usize = draw_range.start as usize;
    let drawn: &[u16] = &indices[start..start + draw_range.count as usize];

    match drawn.iter().copied().find(|index| *index as usize >= vertex_count) {
      Some(index) => Err(VisualSkip::malformed(format!(
        "Drawn range references vertex {index}, past the {vertex_count} the vertex chunk holds"
      ))),
      None => Ok(()),
    }
  }

  fn convert_window(window: &OgfSlideWindow) -> VisualSlideWindow {
    VisualSlideWindow {
      offset: window.offset,
      triangle_count: window.num_tris as u32,
      vertex_count: window.num_verts as u32,
    }
  }
}

/// Reverse the winding of every triangle in place.
///
/// Mirroring Z to reach three.js space flips the orientation of every triangle, and swapping the
/// second and third index of each triple restores it. Reversing the array as a whole would give the
/// same per triangle winding while moving every triangle, which silently invalidates every detail
/// table offset into the buffer.
pub(crate) fn reverse_triangle_winding(indices: &mut [u16]) {
  for triangle in indices.chunks_exact_mut(3) {
    triangle.swap(1, 2);
  }
}

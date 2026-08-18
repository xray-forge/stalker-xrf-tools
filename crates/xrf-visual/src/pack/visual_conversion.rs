use xrf_db::{OgfBox, OgfSphere, Vector3d};

use crate::data::visual_bounds::{VisualBounds, VisualBox, VisualSphere};

/// Convert one X-Ray position or direction into three.js space.
///
/// X-Ray renders left handed and three.js is right handed, and negating Z is the mirror that
/// reconciles them. A mirror reverses the orientation of every triangle, which is why geometry that
/// goes through here must also have its winding swapped by the packer's triangle-winding correction.
pub fn convert_vector(vector: &Vector3d) -> Vector3d {
  Vector3d {
    x: vector.x,
    y: vector.y,
    z: -vector.z,
  }
}

/// Convert one texture coordinate pair, which is to leave it alone.
///
/// OGF stores V running downwards, as Direct3D samples it, and the obvious conclusion is that three.js needs it flipped.
/// That is true only of a texture three.js flipped on upload, and every texture here is a compressed DDS, which it cannot
/// flip: `CompressedTexture` defaults `flipY` to false. The rows therefore reach the GPU in the order the file stores
/// them - top first - so V=0 samples the top of the image exactly as Direct3D means it to, and a flip here would render
/// every texture upside down.
///
/// Kept as a named conversion rather than dropped at the call site because the flip is the intuitive thing to do and
/// somebody will reach for it again; this is where the reason not to lives. Proven on `wpn\wpn_ammo`, whose printed label
/// reads the right way up only without it.
pub fn convert_texture_coordinates(u: f32, v: f32) -> (f32, f32) {
  (u, v)
}

/// Convert the extent an OGF header declares, so it compares against measured geometry.
///
/// Mirroring Z swaps which face of the box is nearest, so the corners are re-ordered rather than
/// converted in place: a box whose `min.z` stayed larger than its `max.z` would read as empty.
pub fn convert_declared_bounds(bounding_box: &OgfBox, bounding_sphere: &OgfSphere) -> VisualBounds {
  let min: Vector3d = convert_vector(&bounding_box.min);
  let max: Vector3d = convert_vector(&bounding_box.max);

  VisualBounds {
    bounding_box: VisualBox {
      min: Vector3d {
        x: min.x,
        y: min.y,
        z: min.z.min(max.z),
      },
      max: Vector3d {
        x: max.x,
        y: max.y,
        z: min.z.max(max.z),
      },
    },
    bounding_sphere: VisualSphere {
      center: convert_vector(&bounding_sphere.position),
      radius: bounding_sphere.radius,
    },
  }
}

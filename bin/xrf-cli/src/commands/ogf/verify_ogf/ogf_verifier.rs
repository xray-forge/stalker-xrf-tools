//! Packs every visual under a path and accounts for what came out.
//!
//! Lives in the CLI rather than in `xrf-visual` so that crate stays a pure renderer projection with no
//! filesystem or reporting surface. A gamedata wide visual check belongs with the other gamedata
//! verifiers, not here.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use walkdir::WalkDir;
use xrf_db::{OgfFile, XRayByteOrder};
use xrf_report::{CheckId, CheckReport, Finding, Report, RuleId, Status};
use xrf_visual::{VisualBounds, VisualDescription, VisualPackage, VisualPacker, VisualSkipCause, VisualSubmesh};

const OGF_EXTENSION: &str = "ogf";

/// Fraction of a declared bounding box's diagonal that geometry may exceed it by before it is worth
/// reporting. Exporters pad declared bounds, and float error is unavoidable, so only a gross
/// disagreement says anything.
const BOUNDS_TOLERANCE_RATIO: f32 = 0.01;
const BOUNDS_TOLERANCE_FLOOR: f32 = 0.001;

/// What a sweep counted, beside what it found.
///
/// These are the numbers that decide whether the premises this work rests on hold: that loose visuals
/// are uniformly version 4 skeletons with skinned vertices, and that a progressive submesh's finest
/// detail level is usually not its whole index buffer.
#[derive(Debug, Default)]
pub struct OgfVerificationCensus {
  pub files: usize,
  pub unreadable_files: usize,
  pub files_without_geometry: usize,
  pub submeshes: usize,
  pub packed_submeshes: usize,
  pub unsupported_submeshes: usize,
  pub malformed_submeshes: usize,
  pub progressive_submeshes: usize,
  pub progressive_submeshes_drawing_part_of_the_buffer: usize,
  pub bounds_disagreements: usize,
  pub versions: BTreeMap<u8, usize>,
  pub root_model_types: BTreeMap<String, usize>,
  pub submesh_model_types: BTreeMap<String, usize>,
  pub vertex_formats: BTreeMap<String, usize>,
}

impl OgfVerificationCensus {
  fn count(map: &mut BTreeMap<String, usize>, key: impl Into<String>) {
    *map.entry(key.into()).or_default() += 1;
  }
}

/// Outcome of a sweep: a finalized report and the counts behind it.
#[derive(Debug)]
pub struct OgfVerificationResult {
  pub census: OgfVerificationCensus,
  pub duration: Duration,
  pub report: Report,
}

pub struct OgfVerifier<'a> {
  root: &'a Path,
}

impl<'a> OgfVerifier<'a> {
  pub fn new(root: &'a Path) -> Self {
    Self { root }
  }

  pub fn run(&self) -> OgfVerificationResult {
    let started_at: Instant = Instant::now();

    let mut census: OgfVerificationCensus = OgfVerificationCensus::default();
    let mut read_findings: Vec<Finding> = Vec::new();
    let mut geometry_findings: Vec<Finding> = Vec::new();
    let mut bounds_findings: Vec<Finding> = Vec::new();

    for path in self.visual_paths() {
      census.files += 1;

      // A read error is a value here, but a panic inside the reader is not: the release profile aborts,
      // which is the right outcome because a panic is a defect rather than a property of the file.
      let file: OgfFile = match OgfFile::read_from_path::<XRayByteOrder, _>(&path) {
        Ok(file) => file,
        Err(error) => {
          census.unreadable_files += 1;
          read_findings.push(Finding::new(
            Self::rule("visuals.read"),
            Some(self.subject(&path)),
            error.to_string(),
          ));

          continue;
        }
      };

      self.census_source_formats(&mut census, &file);

      let package: VisualPackage = VisualPacker::pack(&file);

      self.census_package(&mut census, &package.description);

      geometry_findings.extend(self.geometry_findings(&path, &package.description));

      if let Some(finding) = self.bounds_finding(&path, &package.description) {
        census.bounds_disagreements += 1;
        bounds_findings.push(finding);
      }

      if package.description.submeshes.iter().all(|it| it.geometry().is_none()) {
        census.files_without_geometry += 1;
      }
    }

    // A rejected file is invalid input, which `Status::from_is_valid` grades as failed. `Status::Error`
    // is reserved for the checker itself breaking, and a reader that breaks panics rather than
    // returning, so the sweep aborts instead of reaching this.
    let read_status: Status = Status::from_is_valid(read_findings.is_empty());
    let geometry_status: Status = if census.malformed_submeshes > 0 {
      Status::Failed
    } else if census.unsupported_submeshes > 0 {
      Status::Incomplete
    } else {
      Status::Passed
    };
    let bounds_status: Status = match bounds_findings.is_empty() {
      true => Status::Passed,
      false => Status::Incomplete,
    };

    let duration: Duration = started_at.elapsed();

    OgfVerificationResult {
      report: Report::new(vec![
        CheckReport::new(Self::check("read"), read_status, Some(duration), read_findings),
        CheckReport::new(
          Self::check("geometry"),
          geometry_status,
          Some(duration),
          geometry_findings,
        ),
        CheckReport::new(Self::check("bounds"), bounds_status, Some(duration), bounds_findings),
      ]),
      census,
      duration,
    }
  }

  /// Every visual the sweep covers: one named file, or every `.ogf` under a directory.
  fn visual_paths(&self) -> Vec<PathBuf> {
    if self.root.is_file() {
      return vec![self.root.to_path_buf()];
    }

    WalkDir::new(self.root)
      .into_iter()
      .filter_map(Result::ok)
      .filter(|entry| entry.file_type().is_file())
      .map(|entry| entry.into_path())
      .filter(|path| {
        path
          .extension()
          .is_some_and(|it| it.eq_ignore_ascii_case(OGF_EXTENSION))
      })
      .collect()
  }

  /// Count what the file declares, before packing has a chance to normalise any of it away.
  fn census_source_formats(&self, census: &mut OgfVerificationCensus, file: &OgfFile) {
    *census.versions.entry(file.header.version).or_default() += 1;

    let sources: Vec<&OgfFile> = match file.children.as_ref().map(|it| it.nested.as_slice()) {
      Some(nested) if !nested.is_empty() => nested.iter().collect(),
      _ => vec![file],
    };

    for source in sources {
      let format: String = match source.geometry.as_ref().and_then(|it| it.vertex_format) {
        Some(format) => format!("{format:#010x}"),
        None => String::from("none"),
      };

      OgfVerificationCensus::count(&mut census.vertex_formats, format);
    }
  }

  fn census_package(&self, census: &mut OgfVerificationCensus, description: &VisualDescription) {
    OgfVerificationCensus::count(&mut census.root_model_types, description.model_type_label.clone());

    for submesh in &description.submeshes {
      census.submeshes += 1;
      OgfVerificationCensus::count(&mut census.submesh_model_types, submesh.model_type_label.clone());

      match submesh.geometry() {
        Some(geometry) => {
          census.packed_submeshes += 1;

          if !geometry.windows.is_empty() {
            census.progressive_submeshes += 1;

            if geometry.draw_range.count < geometry.index_count {
              census.progressive_submeshes_drawing_part_of_the_buffer += 1;
            }
          }
        }
        None => match submesh.skipped().map(|(cause, _)| cause) {
          Some(VisualSkipCause::Unsupported) => census.unsupported_submeshes += 1,
          Some(VisualSkipCause::Malformed) => census.malformed_submeshes += 1,
          None => {}
        },
      }
    }
  }

  fn geometry_findings(&self, path: &Path, description: &VisualDescription) -> Vec<Finding> {
    description
      .submeshes
      .iter()
      .filter_map(|submesh: &VisualSubmesh| {
        let (cause, reason) = submesh.skipped()?;
        let rule: &str = match cause {
          VisualSkipCause::Unsupported => "visuals.geometry.unsupported",
          VisualSkipCause::Malformed => "visuals.geometry.malformed",
        };

        Some(Finding::new(
          Self::rule(rule),
          Some(format!("{}#{}", self.subject(path), submesh.index)),
          reason,
        ))
      })
      .collect()
  }

  /// Report geometry that reaches outside the extent its header declares.
  ///
  /// Only that direction matters: declared bounds are routinely padded, so geometry sitting well
  /// inside them says nothing, while geometry outside them means the engine would cull a model before
  /// it left the screen.
  fn bounds_finding(&self, path: &Path, description: &VisualDescription) -> Option<Finding> {
    let computed: &VisualBounds = description.computed_bounds.as_ref()?;
    let declared: &VisualBounds = &description.declared_bounds;

    let diagonal: f32 = (declared.bounding_box.max.x - declared.bounding_box.min.x).abs()
      + (declared.bounding_box.max.y - declared.bounding_box.min.y).abs()
      + (declared.bounding_box.max.z - declared.bounding_box.min.z).abs();
    let tolerance: f32 = (diagonal * BOUNDS_TOLERANCE_RATIO).max(BOUNDS_TOLERANCE_FLOOR);

    let excess: f32 = [
      declared.bounding_box.min.x - computed.bounding_box.min.x,
      declared.bounding_box.min.y - computed.bounding_box.min.y,
      declared.bounding_box.min.z - computed.bounding_box.min.z,
      computed.bounding_box.max.x - declared.bounding_box.max.x,
      computed.bounding_box.max.y - declared.bounding_box.max.y,
      computed.bounding_box.max.z - declared.bounding_box.max.z,
    ]
    .into_iter()
    .fold(0.0, f32::max);

    if excess <= tolerance {
      return None;
    }

    Some(Finding::new(
      Self::rule("visuals.bounds.outside"),
      Some(self.subject(path)),
      format!(
        "Geometry reaches {excess} past the declared bounding box, tolerance {tolerance}. \
         Declared {:?} to {:?}, measured {:?} to {:?}",
        declared.bounding_box.min, declared.bounding_box.max, computed.bounding_box.min, computed.bounding_box.max
      ),
    ))
  }

  /// Subject of a finding: the path relative to the swept root, with forward slashes, matching how
  /// gamedata verification names assets.
  fn subject(&self, path: &Path) -> String {
    path
      .strip_prefix(self.root)
      .unwrap_or(path)
      .to_string_lossy()
      .replace('\\', "/")
  }

  fn check(id: &str) -> CheckId {
    CheckId::new(id).expect("Expected a non-empty check id")
  }

  fn rule(id: &str) -> RuleId {
    RuleId::new(id).expect("Expected a non-empty rule id")
  }
}

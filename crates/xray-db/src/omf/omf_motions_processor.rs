use std::collections::{HashMap, HashSet};

use xray_error::{XRayError, XRayResult};
use xray_utils::assert_equal;

use crate::omf::omf_file::OmfFile;

/// Editing operations over the motion set of an omf file.
pub struct OmfMotionsProcessor {}

impl OmfMotionsProcessor {
  /// Keep only motions whose definition name is accepted by the predicate.
  ///
  /// Surviving definitions get their `motion` field renumbered to the new ordinal to keep the
  /// identity relation the game files rely on.
  ///
  /// Returns the count of removed motions.
  pub fn retain_motions(file: &mut OmfFile, predicate: impl Fn(&str) -> bool) -> XRayResult<usize> {
    Self::assert_motions_are_paired(file, "filtering")?;

    let retained: Vec<bool> = file
      .parameters
      .motions
      .iter()
      .map(|it| predicate(&it.name))
      .collect();

    let removed_count: usize = retained.iter().filter(|it| !**it).count();
    let mut retained_iter = retained.iter();

    file.parameters.motions.retain(|_| {
      *retained_iter
        .next()
        .expect("Retained flag for each motion definition")
    });

    let mut retained_iter = retained.iter();

    file
      .motions
      .motions
      .retain(|_| *retained_iter.next().expect("Retained flag for each motion"));

    for (ordinal, definition) in file.parameters.motions.iter_mut().enumerate() {
      definition.motion = u16::try_from(ordinal).map_err(|_| {
        XRayError::new_invalid_error("Motions count exceeds the supported range after filtering")
      })?;
    }

    Ok(removed_count)
  }

  /// Rename motions using the provided old name to new name map.
  ///
  /// Both the definition name, which the engine uses as the lookup key, and the motion payload name
  /// are updated, because the engine asserts the two match at the same ordinal.
  ///
  /// Returns the count of renamed motions.
  pub fn rename_motions(
    file: &mut OmfFile,
    renames: &HashMap<String, String>,
  ) -> XRayResult<usize> {
    Self::assert_motions_are_paired(file, "renaming")?;

    let mut renamed_count: usize = 0;

    for (definition, motion) in file
      .parameters
      .motions
      .iter_mut()
      .zip(file.motions.motions.iter_mut())
    {
      if let Some(renamed) = renames.get(&definition.name) {
        definition.name.clone_from(renamed);
        renamed_count += 1;
      }

      // Keep the payload name aligned with the definition name even when they diverged in source.
      motion.name.clone_from(&definition.name);
    }

    Self::assert_motion_names_are_unique(file)?;

    Ok(renamed_count)
  }

  /// Guard that definitions and payloads can be treated as ordinal pairs.
  fn assert_motions_are_paired(file: &OmfFile, operation: &str) -> XRayResult {
    assert_equal(
      file.parameters.motions.len(),
      file.motions.motions.len(),
      &format!("Expect matching motions and motion definitions count before {operation}"),
    )
  }

  /// Guard that no two motions share a name, which would make one of them unreachable.
  fn assert_motion_names_are_unique(file: &OmfFile) -> XRayResult {
    let mut seen: HashSet<&str> = HashSet::new();

    for definition in &file.parameters.motions {
      if !seen.insert(&definition.name) {
        return Err(XRayError::new_invalid_error(format!(
          "Motion name '{}' is duplicated, motion names must be unique within a file",
          definition.name
        )));
      }
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use std::collections::HashMap;

  use xray_error::XRayResult;

  use crate::data::ogf::ogf_motion::OgfMotion;
  use crate::data::ogf::ogf_motion_definition::OgfMotionDefinition;
  use crate::data::ogf::ogf_part::OgfPart;
  use crate::omf::chunks::omf_motions_chunk::OmfMotionsChunk;
  use crate::omf::chunks::omf_parameters_chunk::OmfParametersChunk;
  use crate::omf::omf_file::OmfFile;
  use crate::omf::omf_motions_processor::OmfMotionsProcessor;

  /// Build a file whose motions are named after the provided list.
  fn new_named_mock(names: &[&str]) -> OmfFile {
    OmfFile {
      parameters: OmfParametersChunk {
        version: 4,
        parts: vec![OgfPart {
          name: String::from("default"),
          bones: vec![(String::from("bip01"), 0)],
        }],
        motions: names
          .iter()
          .enumerate()
          .map(|(ordinal, name)| {
            let mut definition: OgfMotionDefinition = OgfMotionDefinition::new_mock(Vec::new());

            definition.name = String::from(*name);
            definition.motion = ordinal as u16;
            definition
          })
          .collect(),
      },
      motions: OmfMotionsChunk {
        motions: names
          .iter()
          .map(|name| OgfMotion {
            name: String::from(*name),
            count: 1,
            flags: 0,
            remaining: vec![1, 2, 3],
          })
          .collect(),
      },
    }
  }

  #[test]
  fn test_retain_motions_filters_both_lists_and_reindexes() -> XRayResult {
    let mut file: OmfFile = new_named_mock(&["ak_74_draw", "aek_draw", "ak_74_idle", "akm_idle"]);

    assert_eq!(
      OmfMotionsProcessor::retain_motions(&mut file, |name| name.starts_with("ak_74_"))?,
      2
    );

    assert_eq!(file.get_motion_names(), vec!["ak_74_draw", "ak_74_idle"]);
    assert_eq!(
      file
        .motions
        .motions
        .iter()
        .map(|it| it.name.as_str())
        .collect::<Vec<_>>(),
      vec!["ak_74_draw", "ak_74_idle"],
      "Expect motion payloads to be filtered alongside definitions"
    );
    assert_eq!(
      file
        .parameters
        .motions
        .iter()
        .map(|it| it.motion)
        .collect::<Vec<_>>(),
      vec![0, 1],
      "Expect surviving definitions to be renumbered to their new ordinal"
    );

    Ok(())
  }

  #[test]
  fn test_retain_motions_keeps_payload_paired_with_definition() -> XRayResult {
    let mut file: OmfFile = new_named_mock(&["first", "second", "third"]);

    file.motions.motions[1].remaining = vec![42];

    OmfMotionsProcessor::retain_motions(&mut file, |name| name == "second")?;

    assert_eq!(
      file
        .motions
        .motions
        .first()
        .expect("Retained motion")
        .remaining,
      vec![42],
      "Expect the payload of the retained motion to survive, not the payload at index zero"
    );

    Ok(())
  }

  #[test]
  fn test_rename_motions_updates_definitions_and_payloads() -> XRayResult {
    let mut file: OmfFile = new_named_mock(&["ak_74_draw", "ak_74_idle_move"]);

    let renames: HashMap<String, String> = HashMap::from([
      (String::from("ak_74_draw"), String::from("ak74_draw")),
      (
        String::from("ak_74_idle_move"),
        String::from("ak74_idle_moving"),
      ),
    ]);

    assert_eq!(OmfMotionsProcessor::rename_motions(&mut file, &renames)?, 2);

    assert_eq!(
      file.get_motion_names(),
      vec!["ak74_draw", "ak74_idle_moving"]
    );
    assert_eq!(
      file
        .motions
        .motions
        .iter()
        .map(|it| it.name.as_str())
        .collect::<Vec<_>>(),
      vec!["ak74_draw", "ak74_idle_moving"],
      "Expect payload names to track definition names, the engine asserts they match"
    );

    Ok(())
  }

  #[test]
  fn test_rename_motions_leaves_unmapped_names() -> XRayResult {
    let mut file: OmfFile = new_named_mock(&["ak_74_draw", "ak_74_idle"]);

    let renames: HashMap<String, String> =
      HashMap::from([(String::from("ak_74_draw"), String::from("ak74_draw"))]);

    assert_eq!(OmfMotionsProcessor::rename_motions(&mut file, &renames)?, 1);
    assert_eq!(file.get_motion_names(), vec!["ak74_draw", "ak_74_idle"]);

    Ok(())
  }

  #[test]
  fn test_rename_motions_rejects_duplicates() {
    let mut file: OmfFile = new_named_mock(&["first", "second"]);

    let renames: HashMap<String, String> =
      HashMap::from([(String::from("first"), String::from("second"))]);

    assert!(
      OmfMotionsProcessor::rename_motions(&mut file, &renames).is_err(),
      "Expect rename producing duplicate names to be rejected"
    );
  }
}

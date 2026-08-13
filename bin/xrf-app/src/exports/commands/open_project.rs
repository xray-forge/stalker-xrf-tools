use tauri::State;
use xrf_export::{ExportsProject, ExportsProjectParser};

use crate::exports::state::ExportsProjectState;
use crate::types::TauriResult;
use crate::utils::error_to_string;

#[cfg_attr(feature = "typescript-bindings", specta::specta(rename = "open_project"))]
#[tauri::command(rename = "open_project")]
pub async fn exports_open_project(
  project_path: &str,
  state: State<'_, ExportsProjectState>,
) -> TauriResult<ExportsProject> {
  log::info!("Parsing externs from project: {project_path}");

  let parser: ExportsProjectParser = ExportsProjectParser::new();
  let project: ExportsProject = parser.parse_project_from_path(project_path).map_err(error_to_string)?;
  *state.project.lock().unwrap() = Some(project.clone());

  Ok(project)
}

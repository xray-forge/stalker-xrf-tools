export enum EArchivesEditorCommand {
  CLOSE_ARCHIVES_PROJECT = "plugin:archives_editor|close_archives_project",
  GET_ARCHIVES_PROJECT = "plugin:archives_editor|get_archives_project",
  HAS_ARCHIVES_PROJECT = "plugin:archives_editor|has_archives_project",
  OPEN_ARCHIVES_PROJECT = "plugin:archives_editor|open_archives_project",
  READ_ARCHIVE_FILE = "plugin:archives_editor|read_archive_file",
  UNPACK_ARCHIVES_PATH = "plugin:archives_editor|unpack_archives_path",
}

export enum EConfigsEditorCommand {
  CHECK_FORMAT_CONFIGS_PATH = "plugin:configs_editor|check_format_configs_path",
  FORMAT_CONFIGS_PATH = "plugin:configs_editor|format_configs_path",
  VERIFY_CONFIGS_PATH = "plugin:configs_editor|verify_configs_path",
}

export enum EIconsEditorCommand {
  CLOSE_EQUIPMENT_SPRITE = "plugin:icons_editor|close_equipment_sprite",
  GET_EQUIPMENT_SPRITE = "plugin:icons_editor|get_equipment_sprite",
  OPEN_EQUIPMENT_SPRITE = "plugin:icons_editor|open_equipment_sprite",
  REOPEN_EQUIPMENT_SPRITE = "plugin:icons_editor|reopen_equipment_sprite",
}

export enum EExportsEditorCommand {
  CLOSE_XR_EFFECTS = "plugin:exports_editor|close_xr_effects",
  CLOSE_XR_EXPORTS = "plugin:exports_editor|close_xr_exports",
  GET_XR_EFFECTS = "plugin:exports_editor|get_xr_effects",
  GET_XR_EXPORTS = "plugin:exports_editor|get_xr_exports",
  HAS_XR_EFFECTS = "plugin:exports_editor|has_xr_effects",
  OPEN_XR_EFFECTS = "plugin:exports_editor|open_xr_effects",
  OPEN_XR_EXPORTS = "plugin:exports_editor|open_xr_exports",
  PARSE_XR_EFFECTS = "plugin:exports_editor|parse_xr_effects",
}

export enum ESpawnsEditorCommand {
  CLOSE_SPAWN_FILE = "plugin:spawns_editor|close_spawn_file",
  EXPORT_SPAWN_FILE = "plugin:spawns_editor|export_spawn_file",
  GET_SPAWN_FILE = "plugin:spawns_editor|get_spawn_file",
  GET_SPAWN_FILE_ALIFE_SPAWNS = "plugin:spawns_editor|get_spawn_file_alife_spawns",
  GET_SPAWN_FILE_ARTEFACT_SPAWNS = "plugin:spawns_editor|get_spawn_file_artefact_spawns",
  GET_SPAWN_FILE_GRAPHS = "plugin:spawns_editor|get_spawn_file_graphs",
  GET_SPAWN_FILE_HEADER = "plugin:spawns_editor|get_spawn_file_header",
  GET_SPAWN_FILE_PATROLS = "plugin:spawns_editor|get_spawn_file_patrols",
  IMPORT_SPAWN_FILE = "plugin:spawns_editor|import_spawn_file",
  OPEN_SPAWN_FILE = "plugin:spawns_editor|open_spawn_file",
  SAVE_SPAWN_FILE = "plugin:spawns_editor|save_spawn_file",
}

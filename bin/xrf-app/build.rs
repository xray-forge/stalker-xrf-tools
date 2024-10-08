use tauri_build::{Attributes, DefaultPermissionRule, WindowsAttributes};

fn main() {
  tauri_build::try_build(apply_inline_plugins(
    Attributes::new()
      .codegen(tauri_build::CodegenContext::new())
      .windows_attributes(WindowsAttributes::new()),
  ))
  .expect("failed to run tauri-build")
}

fn apply_inline_plugins(attributes: Attributes) -> Attributes {
  attributes
    .plugin(
      "archives-editor",
      tauri_build::InlinedPlugin::new()
        .commands(&[
          "close_archives_project",
          "get_archives_project",
          "has_archives_project",
          "open_archives_project",
          "read_archive_file",
          "unpack_archives_path",
        ])
        .default_permission(DefaultPermissionRule::AllowAllCommands),
    )
    .plugin(
      "configs-editor",
      tauri_build::InlinedPlugin::new()
        .commands(&[
          "check_format_configs_path",
          "format_configs_path",
          "verify_configs_path",
        ])
        .default_permission(DefaultPermissionRule::AllowAllCommands),
    )
    .plugin(
      "exports-editor",
      tauri_build::InlinedPlugin::new()
        .commands(&[
          "open_xr_effects",
          "parse_xr_effects",
          "close_xr_effects",
          "has_xr_effects",
          "close_xr_exports",
          "open_xr_exports",
          "get_xr_effects",
          "get_xr_exports",
        ])
        .default_permission(DefaultPermissionRule::AllowAllCommands),
    )
    .plugin(
      "icons-editor",
      tauri_build::InlinedPlugin::new()
        .commands(&[
          "close_equipment_sprite",
          "get_equipment_sprite",
          "open_equipment_sprite",
          "reopen_equipment_sprite",
          "pack_equipment",
        ])
        .default_permission(DefaultPermissionRule::AllowAllCommands),
    )
    .plugin(
      "spawns-editor",
      tauri_build::InlinedPlugin::new()
        .commands(&[
          "export_spawn_file",
          "close_spawn_file",
          "get_spawn_file",
          "get_spawn_file_alife_spawns",
          "get_spawn_file_artefact_spawns",
          "get_spawn_file_graphs",
          "get_spawn_file_header",
          "get_spawn_file_patrols",
          "has_spawn_file",
          "import_spawn_file",
          "open_spawn_file",
          "save_spawn_file",
        ])
        .default_permission(DefaultPermissionRule::AllowAllCommands),
    )
    .plugin(
      "translations-editor",
      tauri_build::InlinedPlugin::new()
        .commands(&[
          "read_translations_project",
          "open_translations_project",
          "get_translations_project",
          "close_translations_project",
        ])
        .default_permission(DefaultPermissionRule::AllowAllCommands),
    )
}

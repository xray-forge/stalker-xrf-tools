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
      "archives",
      tauri_build::InlinedPlugin::new()
        .commands(&[
          "close_project",
          "extract_file",
          "extract_directory",
          "get_project",
          "has_project",
          "open_project",
          "read_audio",
          "read_file",
          "read_image",
          "unpack_directory",
        ])
        .default_permission(DefaultPermissionRule::AllowAllCommands),
    )
    .plugin(
      "configs",
      tauri_build::InlinedPlugin::new()
        .commands(&["check_directory_format", "format_directory", "verify_directory"])
        .default_permission(DefaultPermissionRule::AllowAllCommands),
    )
    .plugin(
      "exports",
      tauri_build::InlinedPlugin::new()
        .commands(&["close_project", "open_project", "get_project", "get_source"])
        .default_permission(DefaultPermissionRule::AllowAllCommands),
    )
    .plugin(
      "equipment-icons",
      tauri_build::InlinedPlugin::new()
        .commands(&[
          "close_sprite",
          "get_sprite",
          "open_sprite",
          "reopen_sprite",
          "pack_sprite",
        ])
        .default_permission(DefaultPermissionRule::AllowAllCommands),
    )
    .plugin(
      "spawn",
      tauri_build::InlinedPlugin::new()
        .commands(&[
          "save_unpacked_directory",
          "close_file",
          "get_file",
          "get_alife_spawns",
          "get_artefact_spawns",
          "get_graphs",
          "get_header",
          "get_path",
          "get_patrols",
          "has_file",
          "open_unpacked_directory",
          "open_file",
          "pack_file",
          "save_file",
          "unpack_file",
        ])
        .default_permission(DefaultPermissionRule::AllowAllCommands),
    )
    .plugin(
      "translations",
      tauri_build::InlinedPlugin::new()
        .commands(&["read_project", "open_project", "get_project", "close_project"])
        .default_permission(DefaultPermissionRule::AllowAllCommands),
    )
}

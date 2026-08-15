// Cargo compiles the application and its build script separately, so both adapters expand this token registry.
// Keep each wire name beside its Rust command path; runtime dispatch, Specta, and ACL generation derive from the pair.
macro_rules! for_each_tauri_command_domain {
  ($consumer:ident) => {
    $consumer! {
      archives => "archives" {
        close_project => crate::archives::commands::close_project::archives_close_project,
        default_pack_config => crate::archives::commands::default_pack_config::archives_default_pack_config,
        export_pack_config => crate::archives::commands::export_pack_config::archives_export_pack_config,
        import_pack_config => crate::archives::commands::import_pack_config::archives_import_pack_config,
        extract_file => crate::archives::commands::extract_file::archives_extract_file,
        extract_directory => crate::archives::commands::extract_directory::archives_extract_directory,
        get_project => crate::archives::commands::get_project::archives_get_project,
        has_project => crate::archives::commands::has_project::archives_has_project,
        open_project => crate::archives::commands::open_project::archives_open_project,
        pack_directory => crate::archives::commands::pack_directory::archives_pack_directory,
        read_audio => crate::archives::commands::read_audio::archives_read_audio,
        read_file => crate::archives::commands::read_file::archives_read_file,
        read_image => crate::archives::commands::read_image::archives_read_image,
        unpack_directory => crate::archives::commands::unpack_directory::archives_unpack_directory,
      }
      configs => "configs" {
        check_directory_format => crate::configs::commands::check_directory_format::configs_check_directory_format,
        format_directory => crate::configs::commands::format_directory::configs_format_directory,
        verify_directory => crate::configs::commands::verify_directory::configs_verify_directory,
      }
      exports => "exports" {
        close_project => crate::exports::commands::close_project::exports_close_project,
        open_project => crate::exports::commands::open_project::exports_open_project,
        get_project => crate::exports::commands::get_project::exports_get_project,
        get_source => crate::exports::commands::get_source::exports_get_source,
      }
      equipment_icons => "equipment-icons" {
        close_sprite => crate::equipment_icons::commands::close_sprite::equipment_icons_close_sprite,
        get_sprite => crate::equipment_icons::commands::get_sprite::equipment_icons_get_sprite,
        open_sprite => crate::equipment_icons::commands::open_sprite::equipment_icons_open_sprite,
        reopen_sprite => crate::equipment_icons::commands::reopen_sprite::equipment_icons_reopen_sprite,
        pack_sprite => crate::equipment_icons::commands::pack_sprite::equipment_icons_pack_sprite,
      }
      spawn => "spawn" {
        save_unpacked_directory => crate::spawn::commands::save_unpacked_directory::spawn_save_unpacked_directory,
        close_file => crate::spawn::commands::close_file::spawn_close_file,
        get_file => crate::spawn::commands::get_file::spawn_get_file,
        get_alife_spawns => crate::spawn::commands::get_alife_spawns::spawn_get_alife_spawns,
        get_artefact_spawns => crate::spawn::commands::get_artefact_spawns::spawn_get_artefact_spawns,
        get_graphs => crate::spawn::commands::get_graphs::spawn_get_graphs,
        get_header => crate::spawn::commands::get_header::spawn_get_header,
        get_patrols => crate::spawn::commands::get_patrols::spawn_get_patrols,
        get_path => crate::spawn::commands::get_path::spawn_get_path,
        has_file => crate::spawn::commands::has_file::spawn_has_file,
        open_unpacked_directory => crate::spawn::commands::open_unpacked_directory::spawn_open_unpacked_directory,
        open_file => crate::spawn::commands::open_file::spawn_open_file,
        pack_file => crate::spawn::commands::pack_file::spawn_pack_file,
        save_file => crate::spawn::commands::save_file::spawn_save_file,
        unpack_file => crate::spawn::commands::unpack_file::spawn_unpack_file,
      }
      system => "system" {
        reveal_path => crate::system::commands::reveal_path::system_reveal_path,
      }
      translations => "translations" {
        close_project => crate::translations::commands::close_project::translations_close_project,
        detect_mode => crate::translations::commands::detect_mode::translations_detect_mode,
        get_project => crate::translations::commands::get_project::translations_get_project,
        open_project => crate::translations::commands::open_project::translations_open_project,
        save_file => crate::translations::commands::save_file::translations_save_file,
        validate_text => crate::translations::commands::validate_text::translations_validate_text,
      }
    }
  };
}

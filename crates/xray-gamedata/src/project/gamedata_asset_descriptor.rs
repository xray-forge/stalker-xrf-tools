use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub struct GamedataAssetDescriptor {
  pub root_index: usize,
  pub hits: usize,
  pub extension: GamedataAssetExtension,
}

impl GamedataAssetDescriptor {
  pub fn new(root_index: usize) -> Self {
    Self {
      root_index,
      hits: 0,
      extension: GamedataAssetExtension::Unknown,
    }
  }

  pub fn new_with_extension(root_index: usize, relative_path: &str) -> Self {
    let extension: GamedataAssetExtension = GamedataAssetExtension::from_path(relative_path);

    if extension == GamedataAssetExtension::Unknown {
      log::warn!("Unknown extension asset: {}", relative_path);
    }

    Self {
      root_index,
      hits: 0,
      extension,
    }
  }
}

impl GamedataAssetDescriptor {
  pub fn add_hit(&mut self) {
    self.hits += 1;
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GamedataAssetExtension {
  Ai,
  Anm,
  CForm,
  Dds,
  Dm,
  Efd,
  EnvMod,
  FogVol,
  Game,
  Geom,
  GeomX,
  Hom,
  Ini,
  Level,
  Lights,
  Ltx,
  Misc,
  Ogf,
  Ogg,
  Ogm,
  Omf,
  Ppe,
  PsStatic,
  SndStatic,
  Script,
  Seq,
  Shader,
  Spawn,
  Thm,
  Unknown,
  Wallmarks,
  Details,
  XrPack,
}

impl GamedataAssetExtension {
  pub fn from_path(path: &str) -> Self {
    if let Some(extension) = PathBuf::from(path.to_lowercase())
      .extension()
      .and_then(OsStr::to_str)
    {
      match extension {
        "ai" => Self::Ai,
        "anm" | "anm1" => Self::Anm,
        "cform" => Self::CForm,
        "dds" => Self::Dds,
        "details" => Self::Details,
        "dm" => Self::Dm,
        "efd" => Self::Efd,
        "env_mod" => Self::EnvMod,
        "fog_vol" => Self::FogVol,
        "game" => Self::Game,
        "geom" => Self::Geom,
        "geomx" => Self::GeomX,
        "hom" => Self::Hom,
        "ini" => Self::Ini,
        "lights" => Self::Lights,
        "log" | "bat" | "py" | "cmd" => Self::Misc,
        "ltx" => Self::Ltx,
        "ogf" => Self::Ogf,
        "ogg" => Self::Ogg,
        "ogm" => Self::Ogm,
        "omf" => Self::Omf,
        "ppe" => Self::Ppe,
        "ps" | "s" | "s_" | "h" | "vs" | "cs" | "hs" | "ds" | "gs" => Self::Shader,
        "ps_static" => Self::PsStatic,
        "script" => Self::Script,
        "seq" | "seq_" => Self::Script,
        "snd_static" => Self::SndStatic,
        "spawn" => Self::Spawn,
        "thm" => Self::Thm,
        "wallmarks" => Self::Wallmarks,
        "xr" => Self::XrPack,
        _ => Self::Unknown,
      }
    } else if path.ends_with("level") {
      Self::Level
    } else if path.ends_with(".s") {
      Self::Shader
    } else {
      Self::Unknown
    }
  }
}

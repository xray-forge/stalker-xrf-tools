use std::ffi::OsStr;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum AssetType {
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

impl AssetType {
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

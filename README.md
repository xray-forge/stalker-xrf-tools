# 🎮 [Stalker XRF Tools](README.md)

[![book](https://img.shields.io/badge/docs-book-blue.svg?style=flat)](https://xray-forge.github.io/stalker-xrf-book)
[![language-rust](https://img.shields.io/badge/language-rust-orange.svg?style=flat)](https://github.com/xray-forge/stalker-xrf-tools/search?l=rust)
[![license](https://img.shields.io/badge/license-MIT-blue.svg?style=flat)](https://github.com/Neloreck/dreamstate/blob/master/LICENSE)
<br/>
![status](https://github.com/xray-forge/stalker-xrf-tools/actions/workflows/build_and_test_windows.yml/badge.svg)
![status](https://github.com/xray-forge/stalker-xrf-tools/actions/workflows/build_and_test_ubuntu.yml/badge.svg)

Set of [utility tools](https://xray-forge.github.io/stalker-xrf-book/tools/tools.html) to assist with xray engine mods
development and debugging. <br/>
Includes UI application (windows) for usability and manual usage and CLI (windows, linux) variant for scripts / CI.

## Application

Documented in [xrf book](https://xray-forge.github.io/stalker-xrf-book/tools/app/app.html).

- [Archive editor](https://xray-forge.github.io/stalker-xrf-book/tools/app/archive_editor.md)
- [Dialog editor](https://xray-forge.github.io/stalker-xrf-book/tools/app/dialog_editor.md)
- [Config editor](https://xray-forge.github.io/stalker-xrf-book/tools/app/config_editor.md)
- [Exports viewer](https://xray-forge.github.io/stalker-xrf-book/tools/app/exports_viewer.md)
- [Icon editor](https://xray-forge.github.io/stalker-xrf-book/tools/app/icon_editor.md)
- [Spawn editor](https://xray-forge.github.io/stalker-xrf-book/tools/app/spawn_editor.md)

<img width="600px" src="https://xray-forge.github.io/stalker-xrf-book/tools/app/images/main_window.png">

## CLI

Documented in [xrf book](https://xray-forge.github.io/stalker-xrf-book/tools/cli/cli.html).

- [Archive commands](https://xray-forge.github.io/stalker-xrf-book/tools/cli/archive.html)
- [Gamedata commands](https://xray-forge.github.io/stalker-xrf-book/tools/cli/gamedata.html)
- [Icons commands](https://xray-forge.github.io/stalker-xrf-book/tools/cli/icons.html)
- [LTX commands](https://xray-forge.github.io/stalker-xrf-book/tools/cli/ltx.html)
- [OGF commands](https://xray-forge.github.io/stalker-xrf-book/tools/cli/ogf.html)
- [OMF commands](https://xray-forge.github.io/stalker-xrf-book/tools/cli/omf.html)
- [Particles commands](https://xray-forge.github.io/stalker-xrf-book/tools/cli/particles.html)
- [Spawn commands](https://xray-forge.github.io/stalker-xrf-book/tools/cli/spawn.html)
- [Translations commands](https://xray-forge.github.io/stalker-xrf-book/tools/cli/translations.html)

## Building

### Requirements

- rust
- cargo-make (`cargo install --force cargo-make`)
- tauri-cli (`cargo install --force tauri-cli@1.5.11`),
  [tauri installation](https://tauri.app/v1/guides/getting-started/prerequisites)

use std::fs;
use std::path::{Path, PathBuf};

use clap::{Arg, ArgAction, ArgMatches, Command, value_parser};
use xrf_output::OutputOptions;

use super::command_reference::GroupReference;
use super::markdown_renderer::ReferenceMarkdownRenderer;
use crate::generic_command::{CommandResult, GenericCommand};
use crate::output::TerminalOutput;

#[derive(Default)]
pub struct GenerateDocsCommand;

impl GenericCommand for GenerateDocsCommand {
  fn name(&self) -> &'static str {
    "generate-docs"
  }

  fn init(&self) -> Command {
    Command::new(self.name())
      .about("Command to generate markdown reference for all CLI commands")
      .arg(
        Arg::new("output")
          .help("Path to fully generated documentation directory")
          .short('o')
          .long("output")
          .required(true)
          .value_parser(value_parser!(PathBuf)),
      )
      .arg(
        Arg::new("check")
          .help("Verify existing documentation is up to date instead of writing it")
          .short('c')
          .long("check")
          .required(false)
          .action(ArgAction::SetTrue),
      )
      .arg(
        Arg::new("silent")
          .help("Disable any logging")
          .short('s')
          .long("silent")
          .required(false)
          .action(ArgAction::SetTrue),
      )
      .arg(
        Arg::new("verbose")
          .help("Turn on verbose logging")
          .short('v')
          .long("verbose")
          .required(false)
          .action(ArgAction::SetTrue),
      )
  }

  fn execute(&self, matches: &ArgMatches) -> CommandResult {
    let output_dir: &PathBuf = matches
      .get_one::<PathBuf>("output")
      .expect("Expected valid output directory to be provided");

    let output: OutputOptions = TerminalOutput::from_options(matches.get_flag("silent"), matches.get_flag("verbose"));

    let groups: Vec<GroupReference> = crate::setup::setup_command_groups()
      .iter()
      .map(GroupReference::from_group)
      .collect();

    let pages: Vec<(String, String)> = ReferenceMarkdownRenderer::render_pages(&groups);

    if matches.get_flag("check") {
      Self::check_pages(output_dir, &pages, &output)
    } else {
      Self::write_pages(output_dir, &pages, &output)
    }
  }
}

impl GenerateDocsCommand {
  /// Replaces generated Markdown pages, including pages left over after a group rename.
  fn write_pages(directory: &Path, pages: &[(String, String)], output: &OutputOptions) -> CommandResult {
    fs::create_dir_all(directory)?;

    for name in Self::list_unexpected_pages(directory, pages) {
      xrf_output::info!(output, "Removing stale documentation page: {name}");
      fs::remove_file(directory.join(name))?;
    }

    for (name, content) in pages {
      fs::write(directory.join(name), content)?;
    }

    xrf_output::info!(
      output,
      "Generated {} documentation pages in {}",
      pages.len(),
      directory.display()
    );

    Ok(())
  }

  fn check_pages(directory: &Path, pages: &[(String, String)], output: &OutputOptions) -> CommandResult {
    let mut stale: Vec<String> = Vec::new();

    for (name, content) in pages {
      match fs::read_to_string(directory.join(name)) {
        // Windows checkouts may materialize committed pages with CRLF endings.
        Ok(existing) if existing.replace("\r\n", "\n") == *content => {}
        Ok(_) => stale.push(format!("outdated: {name}")),
        Err(_) => stale.push(format!("missing: {name}")),
      }
    }

    for name in Self::list_unexpected_pages(directory, pages) {
      stale.push(format!("unexpected: {name}"));
    }

    if stale.is_empty() {
      xrf_output::info!(
        output,
        "CLI documentation in {} is up to date ({} pages)",
        directory.display(),
        pages.len()
      );

      Ok(())
    } else {
      Err(
        format!(
          "CLI documentation in {} is stale, regenerate it with 'xrf-cli generate-docs':\n{}",
          directory.display(),
          stale.join("\n")
        )
        .into(),
      )
    }
  }

  fn list_unexpected_pages(directory: &Path, pages: &[(String, String)]) -> Vec<String> {
    let Ok(entries) = fs::read_dir(directory) else {
      return Vec::new();
    };

    entries
      .flatten()
      .filter_map(|entry| entry.file_name().into_string().ok())
      .filter(|name| name.ends_with(".md") && !pages.iter().any(|(page, _)| page == name))
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use clap::ArgMatches;

  use super::GenerateDocsCommand;
  use crate::generic_command::GenericCommand;

  fn parse_matches(extra: &[&str]) -> ArgMatches {
    let mut arguments = vec!["generate-docs", "--output", "docs"];

    arguments.extend_from_slice(extra);

    GenerateDocsCommand.init().try_get_matches_from(arguments).unwrap()
  }

  #[test]
  fn check_mode_is_disabled_by_default() {
    assert!(!parse_matches(&[]).get_flag("check"));
    assert!(parse_matches(&["--check"]).get_flag("check"));
  }
}

use anyhow::{Context, Result};
use asyncgit::sync::diff::{get_diff, DiffLineType, DiffOptions};
use asyncgit::sync::RepoPath;

pub fn get_file_diff(file_path: &str) -> Result<Vec<(DiffLineType, String)>> {
  let repo_path = RepoPath::from(".");
  let diff_options = DiffOptions::default();
  let diff =
    get_diff(&repo_path, file_path, false, Some(diff_options)).context("Failed to get diff")?;

  let lines: Vec<(DiffLineType, String)> = diff
    .hunks
    .into_iter()
    .flat_map(|hunk| {
      hunk
        .lines
        .into_iter()
        .map(|line| (line.line_type, line.content.into_string()))
    })
    .collect();

  Ok(lines)
}

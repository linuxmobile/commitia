use asyncgit::sync::diff::{get_diff, DiffLineType, DiffOptions};
use asyncgit::sync::status::{get_status, StatusItemType, StatusType};
use asyncgit::sync::RepoPath;

pub fn get_git_status_files() -> Result<Vec<(String, StatusItemType)>, Box<dyn std::error::Error>> {
  let repo_path = RepoPath::from(".");
  let staged = get_status(&repo_path, StatusType::Stage, None)?;
  let unstaged = get_status(&repo_path, StatusType::WorkingDir, None)?;

  let mut all_files = Vec::new();
  all_files.extend(staged.iter().map(|e| (e.path.clone(), e.status)));
  all_files.extend(unstaged.iter().map(|e| (e.path.clone(), e.status)));

  all_files.sort_by(|a, b| a.0.cmp(&b.0));
  all_files.dedup_by(|a, b| a.0 == b.0);

  Ok(all_files)
}

pub fn get_file_diff(
  file_path: &str,
) -> Result<Vec<(DiffLineType, String)>, Box<dyn std::error::Error>> {
  let repo_path = RepoPath::from(".");
  let diff_options = DiffOptions::default(); // Initialize DiffOptions correctly
  let diff = get_diff(&repo_path, file_path, false, Some(diff_options))?;
  let lines: Vec<(DiffLineType, String)> = diff
    .hunks
    .iter()
    .flat_map(|hunk| {
      hunk
        .lines
        .iter()
        .map(|line| (line.line_type, line.content.to_string()))
    })
    .collect();
  Ok(lines)
}

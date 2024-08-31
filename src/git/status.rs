use anyhow::{Context, Result};
use asyncgit::sync::status::{get_status, StatusItemType, StatusType};
use asyncgit::sync::RepoPath;

pub fn get_git_status_files() -> Result<Vec<(String, StatusItemType)>> {
  let repo_path = RepoPath::from(".");
  let staged =
    get_status(&repo_path, StatusType::Stage, None).context("Failed to get staged status")?;
  let unstaged = get_status(&repo_path, StatusType::WorkingDir, None)
    .context("Failed to get unstaged status")?;

  let mut all_files: Vec<(String, StatusItemType)> = staged
    .into_iter()
    .chain(unstaged)
    .map(|e| (e.path, e.status))
    .collect();

  all_files.sort_by(|a, b| a.0.cmp(&b.0));
  all_files.dedup_by(|a, b| a.0 == b.0);

  Ok(all_files)
}

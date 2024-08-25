use asyncgit::sync::status::{get_status, StatusType};
use asyncgit::sync::RepoPath;

pub fn get_staged_files() -> Result<Vec<String>, Box<dyn std::error::Error>> {
  let repo_path = RepoPath::from(".");
  let status = get_status(&repo_path, StatusType::Stage, None)?;
  let files: Vec<String> = status.iter().map(|entry| entry.path.clone()).collect();
  Ok(files)
}

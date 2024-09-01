mod diff;
mod status;

use anyhow::Result;

pub use diff::get_file_diff;
pub use status::get_git_status_files;

pub fn get_repo_name() -> Result<String> {
  let current_dir = std::env::current_dir()?;
  Ok(
    current_dir
      .file_name()
      .and_then(|name| name.to_str())
      .unwrap_or("unknown")
      .to_string(),
  )
}

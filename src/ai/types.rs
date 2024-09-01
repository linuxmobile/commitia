use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileDiff {
  pub path: String,
  pub diff: String,
}

#[derive(Serialize, Deserialize)]
pub struct CommitContext {
  pub repo_name: String,
  pub files: Vec<FileDiff>,
}

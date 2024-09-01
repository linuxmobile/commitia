mod ai;
mod config;
mod git;
mod ui;

use anyhow::{Context, Result};
use async_std;
use config::config_exists;
use ui::run_ui;

#[async_std::main]
async fn main() -> Result<()> {
  if !config_exists() {
    run_ui(true)
      .await
      .context("Failed to run UI for initial setup")?;
  } else {
    run_ui(false)
      .await
      .context("Failed to run UI for normal operation")?;
  }
  Ok(())
}

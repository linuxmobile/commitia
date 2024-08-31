mod config;
mod git;
mod ui;

use anyhow::{Context, Result};
use config::config_exists;
use ui::run_ui;

fn main() -> Result<()> {
  if !config_exists() {
    run_ui(true).context("Failed to run UI for initial setup")?;
  } else {
    run_ui(false).context("Failed to run UI for normal operation")?;
  }
  Ok(())
}

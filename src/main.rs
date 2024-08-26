mod config;
mod git;
mod ui;

use config::config_exists;
use ui::run_ui;

fn main() {
  if !config_exists() {
    if let Err(e) = run_ui(true) {
      eprintln!("Error: {}", e);
    }
  } else {
    if let Err(e) = run_ui(false) {
      eprintln!("Error: {}", e);
    }
  }
}

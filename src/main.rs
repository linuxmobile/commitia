mod config;
mod git;
mod ui;

use ui::run_ui;

fn main() {
  if let Err(e) = run_ui() {
    eprintln!("Error: {}", e);
  }
}

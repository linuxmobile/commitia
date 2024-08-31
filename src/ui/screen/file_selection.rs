use crate::ui::app::App;
use crate::ui::widgets::{FileDiff, FileSelector};
use ratatui::{
  layout::{Constraint, Direction, Layout},
  Frame,
};

pub fn draw_file_selection(f: &mut Frame, app: &App, selected_index: usize) {
  let size = f.area();
  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
    .split(size);

  FileSelector::render(f, chunks[0], &app.staged_files, selected_index);

  if let Some(_) = &app.selected_file {
    FileDiff::render(f, chunks[1], &app.file_diff);
  }
}

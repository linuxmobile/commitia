use crate::ui::app::App;
use crate::ui::widgets::{FileDiff, FileSelector};
use ratatui::{
  layout::{Constraint, Direction, Layout},
  style::{Color, Style},
  widgets::{Block, Borders, Paragraph},
  Frame,
};

pub fn draw_file_selection(f: &mut Frame, app: &App) {
  let size = f.area();
  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
    .split(size);

  if app.staged_files.is_empty() {
    let message = Paragraph::new("There is nothing to commit. Working tree clean.")
      .style(Style::default().fg(Color::Yellow))
      .block(Block::default().borders(Borders::ALL).title("Git Status"));
    f.render_widget(message, chunks[0]);
  } else {
    FileSelector::render(f, chunks[0], &app.staged_files, app.selected_index);
  }

  if let Some(_) = &app.selected_file {
    FileDiff::render(f, chunks[1], &app.file_diff);
  }
}

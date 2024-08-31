use crate::ui::app::{ActiveColumn, App};
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
    .direction(Direction::Vertical)
    .constraints([Constraint::Min(0), Constraint::Length(1)])
    .split(size);

  let main_chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
    .split(chunks[0]);

  if app.staged_files.is_empty() {
    let message = Paragraph::new("There is nothing to commit. Working tree clean.")
      .style(Style::default().fg(Color::Yellow))
      .block(Block::default().borders(Borders::ALL).title("Git Status"));
    f.render_widget(message, main_chunks[0]);
  } else {
    FileSelector::render(
      f,
      main_chunks[0],
      &app.staged_files,
      app.selected_index,
      app.active_column == ActiveColumn::Sidebar,
    );
  }

  if let Some(selected_index) = app.selected_file {
    if let Some((file, _)) = app.staged_files.get(selected_index) {
      FileDiff::render(
        f,
        main_chunks[1],
        &app.file_diff,
        app.active_column == ActiveColumn::Main,
      );
    }
  }

  let hint = Paragraph::new("Tab: Switch columns | q: Quit")
    .style(Style::default().fg(Color::Gray))
    .alignment(ratatui::layout::Alignment::Center);
  f.render_widget(hint, chunks[1]);
}

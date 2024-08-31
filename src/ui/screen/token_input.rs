use crate::ui::app::{App, InputMode};
use crate::ui::utils::centered_rect;
use ratatui::{
  layout::Alignment,
  style::{Color, Style},
  widgets::{Block, Borders, Paragraph},
  Frame,
};

pub fn draw_token_input(f: &mut Frame, app: &App) {
  let size = f.area();
  let area = centered_rect(60, 20, size);

  let input = Paragraph::new(app.token.as_str())
    .style(match app.input_mode {
      InputMode::Normal => Style::default(),
      InputMode::Editing => Style::default().fg(Color::Yellow),
      InputMode::SelectingFiles => Style::default().fg(Color::Cyan),
    })
    .block(
      Block::default()
        .borders(Borders::ALL)
        .title("Enter Google AI API token"),
    )
    .alignment(Alignment::Left);

  f.render_widget(input, area);
}

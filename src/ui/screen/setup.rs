use crate::ui::utils::centered_rect;
use ratatui::{
  layout::{Alignment, Rect},
  style::{Color, Style},
  widgets::{Block, Borders, Paragraph, Wrap},
  Frame,
};

pub fn draw_setup_screen(f: &mut Frame, additional_message: &str) {
  let size = f.area();
  let area = centered_rect(80, 50, size);

  let additional_paragraph = Paragraph::new(additional_message)
    .style(Style::default().fg(Color::White))
    .block(
      Block::default()
        .borders(Borders::ALL)
        .title("Welcome to Commitia"),
    )
    .alignment(Alignment::Left)
    .wrap(Wrap { trim: true });

  f.render_widget(additional_paragraph, area);

  let options_area = Rect {
    x: area.x,
    y: area.y + area.height + 1,
    width: area.width,
    height: 3,
  };

  let options_paragraph = Paragraph::new("Press 'c' to continue or 'q' to quit.")
    .style(Style::default().fg(Color::Yellow))
    .alignment(Alignment::Center);

  f.render_widget(options_paragraph, options_area);
}

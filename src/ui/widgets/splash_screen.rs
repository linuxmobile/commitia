use ratatui::{
  layout::Rect,
  style::{Color, Modifier, Style},
  widgets::{Block, Borders, Gauge},
  Frame,
};
use tui_big_text::{BigTextBuilder, PixelSize};

pub struct SplashScreen;

impl SplashScreen {
  pub fn render(f: &mut Frame, area: Rect, welcome_message: &str, progress: f64) {
    let big_text = BigTextBuilder::default()
      .pixel_size(PixelSize::Full)
      .style(Style::default().fg(Color::White))
      .lines(vec![welcome_message.into()])
      .build();
    f.render_widget(big_text, area);

    let gauge_area = Rect {
      x: area.x,
      y: area.y + area.height + 1,
      width: area.width,
      height: 3,
    };
    let gauge = Gauge::default()
      .block(Block::default().borders(Borders::ALL).title("Loading"))
      .gauge_style(
        Style::default()
          .fg(Color::White)
          .bg(Color::Black)
          .add_modifier(Modifier::BOLD),
      )
      .percent((progress * 100.0) as u16);
    f.render_widget(gauge, gauge_area);
  }
}

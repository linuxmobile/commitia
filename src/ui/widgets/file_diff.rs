use asyncgit::sync::diff::DiffLineType;
use ratatui::{
  layout::Rect,
  style::{Color, Style},
  widgets::{Block, Borders, List, ListItem},
  Frame,
};

pub struct FileDiff;

impl FileDiff {
  pub fn render(f: &mut Frame, area: Rect, diff: &[(DiffLineType, String)], is_active: bool) {
    let items: Vec<ListItem> = diff
      .iter()
      .map(|(line_type, line)| {
        let style = match line_type {
          DiffLineType::Add => Style::default().fg(Color::Green),
          DiffLineType::Delete => Style::default().fg(Color::Red),
          DiffLineType::Header => Style::default().fg(Color::Cyan),
          _ => Style::default().fg(Color::White),
        };
        ListItem::new(line.as_str()).style(style)
      })
      .collect();

    let diff_list = List::new(items)
      .block(Block::default().borders(Borders::ALL).title("File Diff"))
      .style(Style::default().fg(if is_active {
        Color::Yellow
      } else {
        Color::White
      }));

    f.render_widget(diff_list, area);
  }
}

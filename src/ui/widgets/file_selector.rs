use asyncgit::sync::status::StatusItemType;
use ratatui::{
  layout::Rect,
  style::{Color, Style},
  widgets::{Block, Borders, List, ListItem},
  Frame,
};

pub struct FileSelector;

impl FileSelector {
  pub fn render(
    f: &mut Frame,
    area: Rect,
    files: &[(String, StatusItemType)],
    selected_index: usize,
  ) {
    let items: Vec<ListItem> = files
      .iter()
      .enumerate()
      .map(|(i, (path, status))| {
        let style = if i == selected_index {
          Style::default().fg(Color::Yellow)
        } else {
          Style::default().fg(Color::White)
        };
        let status_str = match status {
          StatusItemType::Modified => " M ",
          StatusItemType::New => "?? ",
          StatusItemType::Deleted => " D ",
          StatusItemType::Renamed => " R ",
          StatusItemType::Typechange => " T ",
          _ => "   ",
        };
        ListItem::new(format!("{} {}", status_str, path)).style(style)
      })
      .collect();

    let files_list = List::new(items)
      .block(Block::default().borders(Borders::ALL).title("Select Files"))
      .style(Style::default().fg(Color::White));

    f.render_widget(files_list, area);
  }
}

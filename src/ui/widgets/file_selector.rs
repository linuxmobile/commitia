use asyncgit::sync::status::StatusItemType;
use ratatui::{
  layout::Rect,
  style::{Color, Style},
  text::Span,
  widgets::{Block, Borders, List, ListItem},
  Frame,
};

pub struct FileSelector;

impl FileSelector {
  pub fn render(
    f: &mut Frame,
    area: Rect,
    files: &[(Box<str>, StatusItemType)],
    selected_index: usize,
    selected_files: &[usize],
    is_active: bool,
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
        let checkbox = if selected_files.contains(&i) {
          "[x] "
        } else {
          "[ ] "
        };
        let content = Span::styled(format!("{}{}{}", checkbox, status_str, path), style);
        ListItem::new(content)
      })
      .collect();

    let files_list = List::new(items)
      .block(Block::default().borders(Borders::ALL).title("Select Files"))
      .style(Style::default().fg(if is_active {
        Color::Yellow
      } else {
        Color::White
      }));

    f.render_widget(files_list, area);
  }
}

use crate::config;
use crate::git::get_staged_files;
use crate::ui::app::{App, InputMode};
use crate::ui::events::{Event, Events};
use crossterm::{
  event::{KeyCode, KeyModifiers},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
  backend::CrosstermBackend,
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  style::{Color, Modifier, Style},
  widgets::{Block, Borders, Gauge, List, ListItem, Paragraph, Wrap},
  Frame, Terminal,
};
use std::io;
use std::time::{Duration, Instant};
use tui_big_text::{BigTextBuilder, PixelSize};
use webbrowser;

fn top_left_rect(_percent_x: u16, percent_y: u16, r: Rect) -> Rect {
  Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Percentage(percent_y), Constraint::Min(0)].as_ref())
    .split(r)[0]
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
  let popup_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints(
      [
        Constraint::Percentage((100 - percent_y) / 2),
        Constraint::Percentage(percent_y),
        Constraint::Percentage((100 - percent_y) / 2),
      ]
      .as_ref(),
    )
    .split(r);

  Layout::default()
    .direction(Direction::Horizontal)
    .constraints(
      [
        Constraint::Percentage((100 - percent_x) / 2),
        Constraint::Percentage(percent_x),
        Constraint::Percentage((100 - percent_x) / 2),
      ]
      .as_ref(),
    )
    .split(popup_layout[1])[1]
}

fn show_file_selector(f: &mut Frame, files: &[String]) {
  let chunks = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Percentage(100)].as_ref())
    .split(f.area());

  let items: Vec<ListItem> = files.iter().map(|f| ListItem::new(f.as_str())).collect();
  let files_list = List::new(items)
    .block(Block::default().borders(Borders::ALL).title("Select Files"))
    .style(Style::default().fg(Color::White));

  f.render_widget(files_list, chunks[0]);
}

pub fn run_ui() -> Result<(), io::Error> {
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let mut app = App::new();
  let events = Events::new(Duration::from_millis(200));

  let welcome_message = "commitia_";
  let additional_message = "This is the initial setup! 🚀\n\
        To get started with Commitia, you'll need to provide your Google AI token.";

  let start_time = Instant::now();
  let splash_duration = Duration::from_secs(4);

  while start_time.elapsed() < splash_duration {
    let progress = start_time.elapsed().as_secs_f64() / splash_duration.as_secs_f64();
    terminal.draw(|frame| {
      let size = frame.area();
      let area = centered_rect(80, 50, size);
      let big_text = BigTextBuilder::default()
        .pixel_size(PixelSize::Full)
        .style(Style::default().fg(Color::White))
        .lines(vec![welcome_message.into()])
        .build();
      frame.render_widget(big_text, area);

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
      frame.render_widget(gauge, gauge_area);
    })?;
    std::thread::sleep(Duration::from_millis(100));
  }

  loop {
    terminal.draw(|f| {
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
        .style(
          Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);

      f.render_widget(options_paragraph, options_area);
    })?;

    match events.next() {
      Ok(Event::Input(key_event)) => match key_event.code {
        KeyCode::Char('c') => match webbrowser::open("https://aistudio.google.com/app/apikey") {
          Ok(_) => {
            app.input_mode = InputMode::Editing;
            break;
          }
          Err(e) => {
            eprintln!("Failed to open the web browser: {}", e);
          }
        },
        KeyCode::Char('q') => {
          disable_raw_mode()?;
          execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
          terminal.show_cursor()?;
          return Ok(());
        }
        _ => {}
      },
      Ok(Event::Tick) => {}
      Err(e) => {
        eprintln!("Error: {:?}", e);
        break;
      }
    }
  }

  loop {
    terminal.draw(|f| {
      let size = f.area();
      let area = top_left_rect(60, 20, size);

      if !app.is_token_set {
        let input = Paragraph::new(app.token.as_ref() as &str)
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
      } else if app.ask_select_files {
        let question = Paragraph::new("Do you want to select the files to commit? (y/n)")
          .style(Style::default().fg(Color::White))
          .block(Block::default().borders(Borders::ALL).title("Select Files"))
          .alignment(Alignment::Left)
          .wrap(Wrap { trim: true });

        f.render_widget(question, area);
      } else if app.input_mode == InputMode::SelectingFiles {
        show_file_selector(f, &app.staged_files);
      }
    })?;

    match events.next() {
      Ok(Event::Input(key_event)) => {
        if app.input_mode == InputMode::Editing {
          match key_event.code {
            KeyCode::Char(c) => {
              app.token.push(c);
            }
            KeyCode::Backspace => {
              app.token.pop();
            }
            KeyCode::Enter => {
              config::save_token(&app.token)?;
              app.is_token_set = true;
              app.input_mode = InputMode::Normal;
              app.ask_select_files = true;
            }
            KeyCode::Esc => {
              app.input_mode = InputMode::Normal;
            }
            _ => {}
          }
        } else if app.ask_select_files {
          match key_event.code {
            KeyCode::Char('y') => {
              app.input_mode = InputMode::SelectingFiles;
              app.ask_select_files = false;
              app.staged_files = get_staged_files().unwrap_or_else(|_| vec![]);
            }
            KeyCode::Char('n') => {
              app.ask_select_files = false;
            }
            _ => {}
          }
        } else {
          match key_event.code {
            KeyCode::Esc | KeyCode::Char('c')
              if key_event.modifiers.contains(KeyModifiers::CONTROL) =>
            {
              break;
            }
            _ => {}
          }
        }
      }
      Ok(Event::Tick) => {}
      Err(e) => {
        eprintln!("Error: {:?}", e);
        break;
      }
    }
  }

  disable_raw_mode()?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
  terminal.show_cursor()?;

  Ok(())
}

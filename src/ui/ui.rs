
use crate::config;
use crate::ui::app::{App, InputMode};
use crate::ui::events::{Event, Events};
use crossterm::{
  event::{DisableMouseCapture, EnableMouseCapture, KeyCode},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
  backend::CrosstermBackend,
  layout::{Constraint, Direction, Layout, Alignment, Rect},
  style::{Color, Style},
  widgets::{Block, Borders, Paragraph, Wrap},
  Terminal,
};
use std::io;
use std::time::Duration;

fn top_left_rect(_percent_x: u16, percent_y: u16, r: Rect) -> Rect {
  Layout::default()
    .direction(Direction::Vertical)
    .constraints(
      [
        Constraint::Percentage(percent_y),
        Constraint::Min(0),
      ]
      .as_ref(),
    )
    .split(r)[0]
}

pub fn run_ui() -> Result<(), io::Error> {
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend)?;

  let mut app = App::new();
  let events = Events::new(Duration::from_millis(200));

  terminal.draw(|f| {
    let size = f.area();
    let area = top_left_rect(60, 20, size);

    let welcome_message = Paragraph::new("Welcome to commitia, a simple CLI tool to commit your changes with a message and push them to your repository.")
      .style(Style::default().fg(Color::White))
      .block(Block::default().borders(Borders::ALL).title("Welcome"))
      .alignment(Alignment::Left)
      .wrap(Wrap { trim: true });

    f.render_widget(welcome_message, area);
  })?;

  std::thread::sleep(Duration::from_secs(5));

  loop {
    terminal.draw(|f| {
      let size = f.area();
      let area = top_left_rect(60, 20, size);

      let input = Paragraph::new(app.token.as_ref() as &str)
        .style(match app.input_mode {
          InputMode::Normal => Style::default(),
          InputMode::Editing => Style::default().fg(Color::Yellow),
        })
        .block(Block::default().borders(Borders::ALL).title("Please enter your Google AI API token to continue."))
        .alignment(Alignment::Left);

      f.render_widget(input, area);
    })?;

    match events.next() {
      Ok(Event::Input(key)) => match key {
        KeyCode::Char(c) => {
          if let InputMode::Editing = app.input_mode {
            app.token.push(c);
          }
        }
        KeyCode::Backspace => {
          if let InputMode::Editing = app.input_mode {
            app.token.pop();
          }
        }
        KeyCode::Enter => {
          if let InputMode::Editing = app.input_mode {
            config::save_token(&app.token).unwrap();
            app.input_mode = InputMode::Normal;
            break;
          }
        }
        KeyCode::Esc => {
          if let InputMode::Editing = app.input_mode {
            app.input_mode = InputMode::Normal;
          } else {
            break;
          }
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

  disable_raw_mode()?;
  execute!(
    terminal.backend_mut(),
    LeaveAlternateScreen,
    DisableMouseCapture
  )?;
  terminal.show_cursor()?;

  Ok(())
}

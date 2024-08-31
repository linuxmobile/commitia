use crate::{
  config::save_token,
  git::get_git_status_files,
  ui::{
    app::{App, InputMode},
    events::{Event, Events},
    input_handler::handle_input,
    screen::{draw_file_selection, draw_setup_screen, draw_splash_screen, draw_token_input},
  },
};
use anyhow::Context;
use crossterm::{
  event::{KeyCode, KeyModifiers},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::{
  io,
  time::{Duration, Instant},
};
use webbrowser;

pub fn run_ui(initial_setup: bool) -> anyhow::Result<()> {
  enable_raw_mode().context("Failed to enable raw mode")?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen).context("Failed to enter alternate screen")?;
  let backend = CrosstermBackend::new(stdout);
  let mut terminal = Terminal::new(backend).context("Failed to create terminal")?;

  let mut app = App::new();
  let events = Events::new(Duration::from_millis(200));

  let welcome_message = "commitia_";
  let additional_message = "This is the initial setup! 🚀\n\
        To get started with Commitia, you'll need to provide your Google AI token.";

  // Show splash screen
  let start_time = Instant::now();
  let splash_duration = Duration::from_secs(3);
  while start_time.elapsed() < splash_duration {
    let progress = start_time.elapsed().as_secs_f64() / splash_duration.as_secs_f64();
    terminal.draw(|f| draw_splash_screen(f, welcome_message, progress))?;
    std::thread::sleep(Duration::from_millis(100));
  }

  if initial_setup {
    // Initial setup flow
    loop {
      terminal.draw(|f| draw_setup_screen(f, additional_message))?;

      if let Ok(Event::Input(key_event)) = events.next() {
        match key_event.code {
          KeyCode::Char('c') => {
            if webbrowser::open("https://aistudio.google.com/app/apikey").is_ok() {
              app.input_mode = InputMode::Editing;
              break;
            }
          }
          KeyCode::Char('q') => return Ok(()),
          _ => {}
        }
      }
    }

    // Token input loop
    loop {
      terminal.draw(|f| draw_token_input(f, &app))?;

      if let Ok(Event::Input(key_event)) = events.next() {
        match app.input_mode {
          InputMode::Editing => match key_event.code {
            KeyCode::Char(c) => app.token.push(c),
            KeyCode::Backspace => {
              app.token.pop();
            }
            KeyCode::Enter => {
              save_token(&app.token).context("Failed to save token")?;
              app.is_token_set = true;
              app.input_mode = InputMode::Normal;
              break;
            }
            KeyCode::Esc => app.input_mode = InputMode::Normal,
            _ => {}
          },
          _ => {
            if key_event.code == KeyCode::Esc
              || (key_event.code == KeyCode::Char('c')
                && key_event.modifiers.contains(KeyModifiers::CONTROL))
            {
              break;
            }
          }
        }
      }
    }
  }

  // Main application loop
  app.is_token_set = true;
  app.staged_files = get_git_status_files().unwrap_or_else(|e| {
    eprintln!("Failed to get git status: {}", e);
    vec![]
  });
  app.input_mode = InputMode::SelectingFiles;
  app.selected_index = 0;

  loop {
    terminal.draw(|f| draw_file_selection(f, &app, app.selected_index))?;

    if let Ok(Event::Input(key_event)) = events.next() {
      if handle_input(&mut app, key_event.code, key_event.modifiers) {
        break;
      }
    }
  }

  // Cleanup
  disable_raw_mode().context("Failed to disable raw mode")?;
  execute!(terminal.backend_mut(), LeaveAlternateScreen)
    .context("Failed to leave alternate screen")?;
  terminal.show_cursor().context("Failed to show cursor")?;

  Ok(())
}

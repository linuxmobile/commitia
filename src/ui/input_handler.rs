use crate::ui::app::{App, InputMode};
use crossterm::event::{KeyCode, KeyModifiers};

pub fn handle_input(app: &mut App, key_code: KeyCode, modifiers: KeyModifiers) -> bool {
  match app.input_mode {
    InputMode::SelectingFiles => handle_selecting_files(app, key_code, modifiers),
    InputMode::Editing => handle_editing(app, key_code),
    InputMode::Normal => handle_normal(app, key_code, modifiers),
  }
}

fn handle_selecting_files(app: &mut App, key_code: KeyCode, modifiers: KeyModifiers) -> bool {
  match key_code {
    KeyCode::Char('j') => {
      if app.selected_index < app.staged_files.len() - 1 {
        app.selected_index += 1;
      }
    }
    KeyCode::Char('k') => {
      if app.selected_index > 0 {
        app.selected_index -= 1;
      }
    }
    KeyCode::Enter => {
      if let Some((path, _)) = app.staged_files.get(app.selected_index) {
        app.selected_file = Some(path.clone());
        app.file_diff = crate::git::get_file_diff(path).unwrap_or_default();
      }
    }
    KeyCode::Esc | KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
      return true;
    }
    _ => {}
  }
  false
}

fn handle_editing(app: &mut App, key_code: KeyCode) -> bool {
  match key_code {
    KeyCode::Char(c) => app.token.push(c),
    KeyCode::Backspace => {
      app.token.pop();
    }
    KeyCode::Enter => {
      crate::config::save_token(&app.token).unwrap();
      app.is_token_set = true;
      app.input_mode = InputMode::Normal;
    }
    KeyCode::Esc => app.input_mode = InputMode::Normal,
    _ => {}
  }
  false
}

fn handle_normal(_app: &mut App, key_code: KeyCode, modifiers: KeyModifiers) -> bool {
  if key_code == KeyCode::Esc
    || (key_code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL))
  {
    return true;
  }
  false
}

use crate::ui::app::{ActiveColumn, App, InputMode};
use crossterm::event::{KeyCode, KeyModifiers};

pub fn handle_input(app: &mut App, key_code: KeyCode, modifiers: KeyModifiers) -> bool {
  if key_code == KeyCode::Char('q') || key_code == KeyCode::Esc {
    return true;
  }

  match app.input_mode {
    InputMode::SelectingFiles => handle_selecting_files(app, key_code, modifiers),
    InputMode::Editing => handle_editing(app, key_code),
    InputMode::Normal => handle_normal(app, key_code, modifiers),
  }
}

fn handle_selecting_files(app: &mut App, key_code: KeyCode, modifiers: KeyModifiers) -> bool {
  match key_code {
    KeyCode::Tab => {
      app.toggle_active_column();
    }
    KeyCode::Enter => {}
    KeyCode::Esc | KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
      return true;
    }
    _ => match app.active_column {
      ActiveColumn::Sidebar => match key_code {
        KeyCode::Char('j') => {
          if app.selected_index < app.staged_files.len().saturating_sub(1) {
            app.selected_index += 1;
            app.selected_file = Some(app.selected_index);
            app.update_file_diff();
            app.diff_scroll = 0;
          }
        }
        KeyCode::Char('k') => {
          if app.selected_index > 0 {
            app.selected_index -= 1;
            app.selected_file = Some(app.selected_index);
            app.update_file_diff();
            app.diff_scroll = 0;
          }
        }
        _ => {}
      },
      ActiveColumn::Main => match key_code {
        KeyCode::Char('j') => {
          app.diff_scroll = app
            .diff_scroll
            .saturating_add(1)
            .min(app.file_diff.len().saturating_sub(1));
        }
        KeyCode::Char('k') => {
          app.diff_scroll = app.diff_scroll.saturating_sub(1);
        }
        KeyCode::Char('d') if modifiers.contains(KeyModifiers::CONTROL) => {
          app.diff_scroll = app
            .diff_scroll
            .saturating_add(10)
            .min(app.file_diff.len().saturating_sub(1));
        }
        KeyCode::Char('u') if modifiers.contains(KeyModifiers::CONTROL) => {
          app.diff_scroll = app.diff_scroll.saturating_sub(10);
        }
        _ => {}
      },
    },
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

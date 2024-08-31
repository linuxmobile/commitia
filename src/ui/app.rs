use asyncgit::sync::diff::DiffLineType;
use asyncgit::sync::status::StatusItemType;

#[derive(PartialEq)]
pub enum InputMode {
  Normal,
  Editing,
  SelectingFiles,
}

#[derive(PartialEq)]
pub enum ActiveColumn {
  Sidebar,
  Main,
}

pub struct App {
  pub active_column: ActiveColumn,
  pub ask_select_files: bool,
  pub file_diff: Vec<(DiffLineType, String)>,
  pub input_mode: InputMode,
  pub is_token_set: bool,
  pub selected_file: Option<usize>,
  pub selected_index: usize,
  pub staged_files: Vec<(String, StatusItemType)>,
  pub token: String,
}

impl App {
  pub fn new() -> App {
    App {
      active_column: ActiveColumn::Sidebar,
      ask_select_files: false,
      file_diff: Vec::new(),
      input_mode: InputMode::Normal,
      is_token_set: false,
      selected_file: None,
      selected_index: 0,
      staged_files: Vec::new(),
      token: String::new(),
    }
  }

  pub fn select_first_file(&mut self) {
    if !self.staged_files.is_empty() {
      self.selected_index = 0;
      self.selected_file = Some(0);
      self.update_file_diff();
    }
  }

  pub fn update_file_diff(&mut self) {
    if let Some(index) = self.selected_file {
      if let Some((file, _)) = self.staged_files.get(index) {
        self.file_diff = crate::git::get_file_diff(file).unwrap_or_default();
      }
    }
  }

  pub fn toggle_active_column(&mut self) {
    self.active_column = match self.active_column {
      ActiveColumn::Sidebar => ActiveColumn::Main,
      ActiveColumn::Main => ActiveColumn::Sidebar,
    };
  }
}

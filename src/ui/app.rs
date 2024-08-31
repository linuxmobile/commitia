use asyncgit::sync::diff::DiffLineType;
use asyncgit::sync::status::StatusItemType;

#[derive(PartialEq)]
pub enum InputMode {
  Normal,
  Editing,
  SelectingFiles,
}

pub struct App {
  pub token: String,
  pub input_mode: InputMode,
  pub is_token_set: bool,
  pub ask_select_files: bool,
  pub staged_files: Vec<(String, StatusItemType)>,
  pub selected_file: Option<String>,
  pub selected_index: usize,
  pub file_diff: Vec<(DiffLineType, String)>,
}

impl App {
  pub fn new() -> App {
    App {
      token: String::new(),
      input_mode: InputMode::Normal,
      is_token_set: false,
      ask_select_files: false,
      staged_files: Vec::new(),
      selected_file: None,
      selected_index: 0,
      file_diff: Vec::new(),
    }
  }
  pub fn select_first_file(&mut self) {
    if !self.staged_files.is_empty() {
      self.selected_index = 0;
      self.selected_file = Some(self.staged_files[0].0.clone());
      self.update_file_diff();
    }
  }

  pub fn update_file_diff(&mut self) {
    if let Some(file) = &self.selected_file {
      self.file_diff = crate::git::get_file_diff(file).unwrap_or_default();
    }
  }
}

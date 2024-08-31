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
}

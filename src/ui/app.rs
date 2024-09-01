use crate::ai::CommitContext;
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
  pub commit_context: Option<CommitContext>,
  pub commit_message: Option<String>,
  pub diff_scroll: usize,
  pub file_diff: Vec<(DiffLineType, String)>,
  pub input_mode: InputMode,
  pub is_token_set: bool,
  pub selected_file: Option<usize>,
  pub selected_files: Vec<usize>,
  pub selected_index: usize,
  pub staged_files: Vec<(Box<str>, StatusItemType)>,
  pub token: String,
}

impl App {
  pub fn new() -> App {
    App {
      active_column: ActiveColumn::Sidebar,
      ask_select_files: false,
      commit_context: None,
      commit_message: None,
      diff_scroll: 0,
      file_diff: Vec::new(),
      input_mode: InputMode::Normal,
      is_token_set: false,
      selected_file: None,
      selected_files: Vec::new(),
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
        self.file_diff = crate::git::get_file_diff(file.as_ref()).unwrap_or_default();
      }
    }
  }

  pub fn toggle_active_column(&mut self) {
    self.active_column = match self.active_column {
      ActiveColumn::Sidebar => ActiveColumn::Main,
      ActiveColumn::Main => ActiveColumn::Sidebar,
    };
  }

  pub fn scroll_diff(&mut self, delta: isize) {
    let new_scroll = self.diff_scroll as isize + delta;
    self.diff_scroll = new_scroll.max(0) as usize;
  }

  pub fn toggle_file_selection(&mut self) {
    if let Some(index) = self.selected_file {
      if self.selected_files.contains(&index) {
        self.selected_files.retain(|&x| x != index);
      } else {
        self.selected_files.push(index);
      }
    }
  }

  pub fn stage_selected_files(&mut self) {
    // This is where we'll implement the staging logic later
    // For now, let's just clear the selection
    self.selected_files.clear();
  }

  pub fn prepare_commit_context(&mut self) -> anyhow::Result<()> {
    let repo_name = crate::git::get_repo_name()?;
    let selected_files: Vec<(String, String)> = self
      .selected_files
      .iter()
      .filter_map(|&index| {
        self.staged_files.get(index).map(|(path, _)| {
          let diff = crate::git::get_file_diff(path).unwrap_or_default();
          let diff_str = diff
            .into_iter()
            .map(|(_, line)| line)
            .collect::<Vec<_>>()
            .join("\n");
          (path.to_string(), diff_str)
        })
      })
      .collect();

    self.commit_context = Some(crate::ai::prepare_commit_context(
      &repo_name,
      &selected_files,
    )?);
    Ok(())
  }

  pub async fn generate_commit_message(&mut self) -> anyhow::Result<()> {
    if let Some(context) = self.commit_context.take() {
      self.commit_message = Some(crate::ai::generate_commit_message(context).await?);
    }
    Ok(())
  }
}

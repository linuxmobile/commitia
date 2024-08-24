#[derive(PartialEq)]
pub enum InputMode {
  Normal,
  Editing,
}

pub struct App {
  pub token: String,
  pub input_mode: InputMode,
  pub show_welcome: bool,
  pub show_popup: bool,
  pub is_token_set: bool,
  pub popup_opened: bool,
}

impl App {
  pub fn new() -> App {
    App {
      token: String::new(),
      input_mode: InputMode::Normal,
      show_welcome: true,
      show_popup: false,
      is_token_set: false,
      popup_opened: false,
    }
  }
}

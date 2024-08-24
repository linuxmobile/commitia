pub enum InputMode {
  Normal,
  Editing,
}

pub struct App {
  pub token: String,
  pub input_mode: InputMode,
  pub show_welcome: bool,
  pub confirm_continue: bool,
  pub show_popup: bool,
}

impl App {
  pub fn new() -> App {
    App {
      token: String::new(),
      input_mode: InputMode::Editing,
      show_welcome: true,
      confirm_continue: false,
      show_popup: false,
    }
  }
}

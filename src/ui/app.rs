
pub struct App {
  pub token: String,
  pub input_mode: InputMode,
}

pub enum InputMode {
  Normal,
  Editing,
}

impl App {
  pub fn new() -> App {
    App {
      token: String::new(),
      input_mode: InputMode::Editing,
    }
  }
}

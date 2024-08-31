use crate::ui::utils::centered_rect;
use crate::ui::widgets::SplashScreen;
use ratatui::Frame;

pub fn draw_splash_screen(f: &mut Frame, welcome_message: &str, progress: f64) {
  let size = f.area();
  let area = centered_rect(80, 50, size);
  SplashScreen::render(f, area, welcome_message, progress);
}

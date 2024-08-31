use crossterm::event::{self, Event as CEvent, KeyEvent};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

pub enum Event<I> {
  Input(I),
  Tick,
}

pub struct Events {
  rx: mpsc::Receiver<Event<KeyEvent>>,
}

impl Events {
  pub fn new(tick_rate: Duration) -> Self {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
      let mut last_tick = Instant::now();
      loop {
        let timeout = tick_rate
          .checked_sub(last_tick.elapsed())
          .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout).unwrap() {
          if let CEvent::Key(key) = event::read().unwrap() {
            if tx.send(Event::Input(key)).is_err() {
              return;
            }
          }
        }

        if last_tick.elapsed() >= tick_rate {
          if tx.send(Event::Tick).is_err() {
            return;
          }
          last_tick = Instant::now();
        }
      }
    });

    Events { rx }
  }

  pub fn next(&self) -> Result<Event<KeyEvent>, mpsc::RecvError> {
    self.rx.recv()
  }
}

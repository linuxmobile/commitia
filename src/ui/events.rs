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
  _tx: mpsc::Sender<Event<KeyEvent>>,
}

impl Events {
  pub fn new(tick_rate: Duration) -> Events {
    let (tx, rx) = mpsc::channel();
    let event_tx = tx.clone();
    let tick_tx = tx.clone();

    thread::spawn(move || {
      let mut last_tick = Instant::now();
      loop {
        let timeout = tick_rate
          .checked_sub(last_tick.elapsed())
          .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout).unwrap() {
          if let CEvent::Key(key) = event::read().unwrap() {
            event_tx.send(Event::Input(key)).unwrap();
          }
        }

        if last_tick.elapsed() >= tick_rate {
          if let Ok(_) = tick_tx.send(Event::Tick) {
            last_tick = Instant::now();
          }
        }
      }
    });

    Events { rx, _tx: tx }
  }

  pub fn next(&self) -> Result<Event<KeyEvent>, mpsc::RecvError> {
    self.rx.recv()
  }
}

use async_std::channel::{self, Receiver};
use async_std::task;
use crossterm::event::{self, Event as CEvent, KeyEvent};
use std::time::{Duration, Instant};

pub enum Event<I> {
  Input(I),
  Tick,
}

pub struct Events {
  rx: Receiver<Event<KeyEvent>>,
}

impl Events {
  pub fn new(tick_rate: Duration) -> Self {
    let (tx, rx) = channel::bounded(100);

    task::spawn(async move {
      let mut last_tick = Instant::now();
      loop {
        let timeout = tick_rate
          .checked_sub(last_tick.elapsed())
          .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout).unwrap() {
          if let CEvent::Key(key) = event::read().unwrap() {
            if tx.send(Event::Input(key)).await.is_err() {
              break;
            }
          }
        }

        if last_tick.elapsed() >= tick_rate {
          if tx.send(Event::Tick).await.is_err() {
            break;
          }
          last_tick = Instant::now();
        }

        async_std::task::sleep(Duration::from_millis(1)).await;
      }
    });

    Events { rx }
  }

  pub async fn next(&mut self) -> Option<Event<KeyEvent>> {
    self.rx.recv().await.ok()
  }
}

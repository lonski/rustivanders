use std::io;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use std::io::{stdin, stdout, Write};
use termion::event::Key;
use termion::event::MouseEvent;
use termion::input::MouseTerminal;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

pub enum Event<I, MI> {
    Input(I),
    MouseInput(MI),
    Tick,
}

/// A small event handler that wrap termion input and tick events. Each event
/// type is handled in its own thread and returned to a common `Receiver`
pub struct Events {
    rx: mpsc::Receiver<Event<Key, MouseEvent>>,
    input_handle: thread::JoinHandle<()>,
    tick_handle: thread::JoinHandle<()>,
}

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub tick_rate: Duration,
}

impl Events {
    pub fn with_config(config: Config) -> Events {
        let (tx, rx) = mpsc::channel();
        let input_handle = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                let mut stdout = MouseTerminal::from(stdout().into_raw_mode().unwrap());
                for c in stdin.events() {
                    let evt = c.unwrap();
                    let _ = match evt {
                        termion::event::Event::Key(key) => tx.send(Event::Input(key)),
                        termion::event::Event::Mouse(me) => tx.send(Event::MouseInput(me)),
                        _ => Ok(()),
                    };
                    stdout.flush().unwrap();
                }
            })
        };
        let tick_handle = {
            thread::spawn(move || loop {
                if let Err(err) = tx.send(Event::Tick) {
                    eprintln!("{}", err);
                    break;
                }
                thread::sleep(config.tick_rate);
            })
        };
        Events {
            rx,
            input_handle,
            tick_handle,
        }
    }

    pub fn next(&self) -> Result<Event<Key, MouseEvent>, mpsc::RecvError> {
        self.rx.recv()
    }
}

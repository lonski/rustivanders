use crate::board::Board;
use crate::events::{Config, Event, Events};
use crate::renderer::Renderer;
use crate::util::Direction;
use std::time::Duration;

use termion::event::Key;

pub struct Rustivanders {
    is_exiting: bool,
    board: Board,
    renderer: Renderer,
}

impl Rustivanders {
    pub fn new() -> Self {
        Rustivanders {
            is_exiting: false,
            board: Board::new(),
            renderer: Renderer::new(),
        }
    }

    pub fn run(&mut self) {
        self.renderer.clear();

        let events = Events::with_config(Config {
            tick_rate: Duration::from_millis(16),
        });

        loop {
            if let Ok(event) = events.next() {
                match event {
                    Event::Input(input) => self.process_input(input),
                    Event::Tick => self.update(),
                }
            }

            self.render();

            if self.is_exiting {
                break;
            }
        }
    }

    fn process_input(&mut self, input: termion::event::Key) {
        match input {
            Key::Esc => self.is_exiting = true,
            Key::Char('q') => self.is_exiting = true,
            Key::Right => self.board.move_player(Direction::Right),
            Key::Left => self.board.move_player(Direction::Left),
            Key::Char(' ') => self.board.player_fire(),
            Key::Char('n') => self.board.next_level(),

            _ => {}
        }
    }

    fn update(&mut self) {
        self.board.update();
    }

    fn render(&mut self) {
        self.renderer.render(&self.board);
    }
}

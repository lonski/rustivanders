use crate::board::Board;
use crate::events::{Config, Event, Events};
use crate::renderer::Renderer;
use crate::util::Direction;
use std::time::Duration;

use std::io::stdout;
use termion::event::{Key, MouseButton, MouseEvent};
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;

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
                    Event::MouseInput(me) => self.process_mouse_input(me),
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
            Key::Down => self.board.move_player(Direction::None),
            Key::Char(' ') => self.board.player_fire(),
            Key::Char('n') => self.board.next_level(),

            _ => {}
        }
    }

    fn process_mouse_input(&mut self, me: termion::event::MouseEvent) {
        match me {
            MouseEvent::Press(btn, x, y) => {
                if btn == MouseButton::Left {
                    if self.board.player.state.pos.x > x as i16 {
                        self.board.move_player(Direction::Left)
                    } else {
                        self.board.move_player(Direction::Right)
                    }
                }
            }
            MouseEvent::Release(v1, v2) => self.board.move_player(Direction::None),

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

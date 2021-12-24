use crate::board::Board;
use crate::point::Point;
use crate::renderer::{Color, ConsoleRenderer, Renderable, Renderer};
use std::boxed::Box;
use std::time::Instant;
use std::{io::Write, time::Duration};

use crossterm::{
    event::{poll, read},
    event::{Event, KeyCode},
};

pub struct Rustivanders {
    is_exiting: bool,
    board: Board,
    renderer: Box<dyn Renderer>,
}

const MS_PER_FRAME: Duration = Duration::from_millis(32);

impl Rustivanders {
    pub fn new<W>(w: W) -> Self
    where
        W: Write + 'static,
    {
        Rustivanders {
            is_exiting: false,
            board: Board::new(),
            renderer: Box::new(ConsoleRenderer::new(w)),
        }
    }

    pub fn run(&mut self) {
        self.renderer.prepare();

        loop {
            let start_time = Instant::now();

            if poll(Duration::from_millis(1)).unwrap_or(false) {
                if let Ok(event) = read() {
                    self.process_input(&event);
                }
            }

            self.update();
            self.render();

            std::thread::sleep(start_time + MS_PER_FRAME - Instant::now());

            if self.is_exiting {
                break;
            }
        }

        self.renderer.cleanup();
    }

    fn process_input(&mut self, event: &Event) {
        match event {
            Event::Key(key_event) => match key_event.code {
                KeyCode::Char(key) => match key {
                    'j' => self.board.move_player_by(&Point::new(0, 1)),
                    'k' => self.board.move_player_by(&Point::new(0, -1)),
                    'l' => self.board.move_player_by(&Point::new(1, 0)),
                    'h' => self.board.move_player_by(&Point::new(-1, 0)),
                    _ => {}
                },
                KeyCode::Left => self.board.move_player_by(&Point::new(-1, 0)),
                KeyCode::Right => self.board.move_player_by(&Point::new(1, 0)),
                KeyCode::Esc => self.is_exiting = true,
                _ => {}
            },
            _ => {}
        }
    }

    fn update(&mut self) {
        self.board.update();
    }

    fn render(&mut self) {
        self.renderer.clear();
        self.board.render(&mut self.renderer);
        self.renderer.flush()
    }
}

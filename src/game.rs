use crate::board::Board;
use crate::renderer::Color;
use crate::renderer::ConsoleRenderer;
use crate::renderer::Renderer;
use std::boxed::Box;
use std::time::Instant;
use std::{io::Write, time::Duration};

use crossterm::{
    event::{poll, read},
    event::{Event, KeyCode},
};

pub struct Rustivanders {
    color: Color,
    is_exiting: bool,
    next_tick: i128,
    board: Board,
    renderer: Box<dyn Renderer>,
}

impl Rustivanders {
    pub fn new<W>(w: W) -> Self
    where
        W: Write + 'static,
    {
        Rustivanders {
            next_tick: 0,
            is_exiting: false,
            color: Color::Red,
            board: Board::new(),
            renderer: Box::new(ConsoleRenderer::new(w)),
        }
    }

    pub fn run(&mut self) {
        self.renderer.prepare();
        let mut last_time = Instant::now();

        loop {
            let current_time = Instant::now();
            let elapsed = current_time - last_time;

            if poll(Duration::from_millis(1)).unwrap_or(false) {
                if let Ok(event) = read() {
                    self.process_input(&event);
                }
            }

            self.update(elapsed);
            self.render();

            last_time = current_time;

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
                    'j' => self.board.move_player_by((1, 0)),
                    'k' => self.board.move_player_by((-1, 0)),
                    'l' => self.board.move_player_by((0, 1)),
                    'h' => self.board.move_player_by((0, -1)),
                    _ => {}
                },
                KeyCode::Left => self.board.move_player_by((0, -1)),
                KeyCode::Right => self.board.move_player_by((0, 1)),
                KeyCode::Esc => self.is_exiting = true,
                _ => {}
            },
            _ => {}
        }
    }

    fn update(&mut self, dt: Duration) {
        self.next_tick -= dt.as_nanos() as i128;
        if self.next_tick <= 0 {
            match self.color {
                Color::Red => self.color = Color::Blue,
                _ => self.color = Color::Red,
            }
            self.next_tick = 1000 * 1000 * 200; //200 ms
        }
    }

    fn render(&mut self) {
        self.renderer.clear();
        self.board.render(&mut self.renderer);
        self.renderer.flush()
    }
}

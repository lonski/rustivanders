use crate::board::Board;
use termion::{raw::IntoRawMode, screen::AlternateScreen};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{
        canvas::{Canvas, Context},
        Block, Borders,
    },
    Terminal,
};

pub trait Renderable {
    fn render(&self, ctx: &mut Context);
}

pub struct Renderer {
    terminal: Terminal<TermionBackend<AlternateScreen<termion::raw::RawTerminal<std::io::Stdout>>>>,
}

impl Renderer {
    pub fn new() -> Self {
        let stdout = std::io::stdout().into_raw_mode().unwrap();
        let stdout = AlternateScreen::from(stdout);
        let backend = TermionBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();
        Renderer { terminal }
    }
}

impl Renderer {
    pub fn clear(&mut self) {
        self.terminal.clear().unwrap();
    }

    pub fn render(&mut self, board: &Board) {
        self.terminal
            .draw(|f| {
                let render_area = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(100)].as_ref())
                    .split(Rect {
                        x: 0,
                        y: 0,
                        width: 96,
                        height: 32,
                    })[0];

                let canvas = Canvas::default()
                    .block(Block::default().borders(Borders::ALL))
                    .paint(|mut ctx| board.render(&mut ctx))
                    .x_bounds([0.0, 94.0])
                    .y_bounds([0.0, 30.0]);
                f.render_widget(canvas, render_area);
            })
            .unwrap();
    }
}

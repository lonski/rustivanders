use crate::point::Point;
use std::io::Write;

use crossterm::{
    cursor, execute, queue, style,
    style::Stylize,
    terminal::{self, disable_raw_mode, enable_raw_mode, ClearType},
};

pub trait Renderer {
    fn prepare(&mut self);
    fn cleanup(&mut self);
    fn clear(&mut self);
    fn flush(&mut self);
    fn draw(&mut self, pos: &Point, c: char, color: Color);
}

pub trait Renderable {
    fn render(&self, renderer: &mut Box<dyn Renderer>);
}

pub struct ConsoleRenderer<W>
where
    W: Write,
{
    w: W,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Blue,
    Green,
    Yellow,
    Grey,
}

impl Color {
    fn to_crossterm(&self) -> crossterm::style::Color {
        match self {
            Color::Red => crossterm::style::Color::Red,
            Color::Green => crossterm::style::Color::Green,
            Color::Blue => crossterm::style::Color::Blue,
            Color::Yellow => crossterm::style::Color::Yellow,
            Color::Grey => crossterm::style::Color::Grey,
        }
    }
}

impl<W> ConsoleRenderer<W>
where
    W: Write,
{
    pub fn new(w: W) -> Self
    where
        W: Write,
    {
        ConsoleRenderer { w }
    }
}

impl<W> Renderer for ConsoleRenderer<W>
where
    W: Write,
{
    fn prepare(&mut self) {
        enable_raw_mode().expect("Failed to enable raw mode");
        execute!(self.w, terminal::EnterAlternateScreen, cursor::Hide,);
    }

    fn cleanup(&mut self) {
        execute!(
            self.w,
            style::ResetColor,
            cursor::Show,
            terminal::LeaveAlternateScreen
        );
        disable_raw_mode().unwrap_or_default();
    }

    fn clear(&mut self) {
        queue!(self.w, terminal::Clear(ClearType::All));
    }

    fn flush(&mut self) {
        self.w.flush().expect("Failed to flush")
    }

    fn draw(&mut self, pos: &Point, c: char, color: Color) {
        queue!(
            self.w,
            cursor::MoveTo(pos.x as u16, pos.y as u16),
            style::Print(style::style(c).with(color.to_crossterm())),
        );
    }
}

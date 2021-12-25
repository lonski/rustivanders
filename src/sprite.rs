use crate::ai::{BulletAi, InvanderAi, PlayerAi};
use crate::point::Point;
use crate::renderer::Renderable;

use tui::style::Color;
use tui::widgets::canvas::Context;

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub symbol: &'static str,
    pub color: Color,
}

impl Cell {
    fn new(symbol: &'static str, color: Color) -> Self {
        Cell { symbol, color }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    None,
}

pub struct SpriteState {
    pub pos: Point,
    pub direction: Direction,
    pub cells: Vec<Vec<Cell>>,
    pub id: u32,
}

pub struct Player {
    pub state: SpriteState,
    pub ai: PlayerAi,
}

impl Player {
    pub fn new(x: i16, y: i16) -> Self {
        Player {
            state: SpriteState {
                id: 0,
                pos: Point::new(x, y),
                direction: Direction::Up,
                cells: vec![
                    vec![
                        Cell::new(" ", Color::Blue),
                        Cell::new("^", Color::LightBlue),
                    ],
                    vec![
                        Cell::new("/", Color::Blue),
                        Cell::new("V", Color::Red),
                        Cell::new("\\", Color::Blue),
                    ],
                ],
            },
            ai: PlayerAi::new(),
        }
    }
}

pub struct Invander {
    pub state: SpriteState,
    pub ai: InvanderAi,
}

impl Invander {
    pub fn new(x: i16, y: i16, x_range: &(i16, i16)) -> Self {
        Invander {
            state: SpriteState {
                id: 0,
                pos: Point::new(x, y),
                direction: Direction::Down,
                cells: vec![
                    vec![
                        Cell::new(" ", Color::Green),
                        Cell::new("_", Color::Green),
                        Cell::new("_", Color::Green),
                        Cell::new("_", Color::Green),
                        Cell::new("_", Color::Green),
                        Cell::new("_", Color::Green),
                        Cell::new(" ", Color::Green),
                    ],
                    vec![
                        Cell::new("<", Color::Green),
                        Cell::new(":", Color::Yellow),
                        Cell::new(":", Color::Yellow),
                        Cell::new(":", Color::Yellow),
                        Cell::new(":", Color::Yellow),
                        Cell::new(":", Color::Yellow),
                        Cell::new(">", Color::Green),
                    ],
                    vec![
                        Cell::new(" ", Color::Green),
                        Cell::new("|", Color::Red),
                        Cell::new(" ", Color::Green),
                        Cell::new("|", Color::Red),
                        Cell::new(" ", Color::Green),
                        Cell::new("|", Color::Red),
                        Cell::new(" ", Color::Green),
                    ],
                ],
            },
            ai: InvanderAi::new(x_range),
        }
    }
}

pub struct Bullet {
    pub state: SpriteState,
    pub ai: BulletAi,
}

impl Bullet {
    pub fn new(x: i16, y: i16, dir: Direction, color: Color) -> Self {
        Bullet {
            state: SpriteState {
                id: 0,
                pos: Point::new(x, y),
                direction: dir,
                cells: vec![vec![Cell::new("*", color)]],
            },
            ai: BulletAi {},
        }
    }
}

impl SpriteState {
    pub fn move_by(&mut self, d: &Point) {
        self.pos += *d;
    }

    pub fn collides(&self, p: &Point) -> bool {
        for (dy, cell_row) in self.cells.iter().enumerate() {
            for (dx, cell) in cell_row.iter().enumerate() {
                if cell.symbol == " " {
                    continue;
                }
                let c = self.pos + Point::new(dx as i16, dy as i16 * -1);
                if *p == c {
                    return true;
                }
            }
        }
        false
    }

    pub fn fire(&self, dir: Direction, color: Color) -> Bullet {
        let start = self.fire_point();
        Bullet::new(start.x, start.y, dir, color)
    }

    pub fn fire_point(&self) -> Point {
        let x = self.pos.x + self.cells[0].len() as i16 / 2;
        let y = match self.direction {
            Direction::Down => self.pos.y - (self.cells.len() as i16),
            _ => self.pos.y,
        };
        Point::new(x, y)
    }
}

impl Renderable for SpriteState {
    fn render(&self, ctx: &mut Context) {
        for (dy, cell_row) in self.cells.iter().enumerate() {
            for (dx, cell) in cell_row.iter().enumerate() {
                if cell.symbol == " " {
                    continue;
                }
                ctx.print(
                    (self.pos.x + dx as i16) as f64,
                    (self.pos.y - dy as i16) as f64,
                    &cell.symbol,
                    cell.color,
                );
            }
        }
    }
}

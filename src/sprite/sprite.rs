use crate::board::UpdateCommand;
use crate::renderer::Renderable;
use crate::sprite::Bullet;
use crate::util::{Direction, Point};

use tui::style::Color;
use tui::widgets::canvas::Context;

pub trait Sprite<'a> {
    fn update(&mut self) -> Vec<UpdateCommand>;

    fn state(&'a self) -> &'a SpriteState;

    fn render(&'a self, ctx: &mut Context) {
        self.state().render(ctx);
    }

    fn collides(&'a self, p: &Point) -> bool {
        self.state().collides(p)
    }

    fn modify_hp(&mut self, _: i16) {}

    fn set_id(&'a mut self, id: u32);
}

#[derive(Debug, Copy, Clone)]
pub struct Cell {
    pub symbol: &'static str,
    pub color: Color,
}

impl Cell {
    pub fn new(symbol: &'static str, color: Color) -> Self {
        Cell { symbol, color }
    }
}

pub struct SpriteState {
    pub hp: u16,
    pub id: u32,
    pub pos: Point,
    pub direction: Direction,
    pub cells: Vec<Vec<Cell>>,
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
                let c = self.pos + Point::new(dx as i16, -(dy as i16));
                if *p == c {
                    return true;
                }
            }
        }
        false
    }

    pub fn fire(&self, dir: Direction, color: Color, bullet_speed: i32) -> Bullet {
        let start = self.fire_point();
        Bullet::new(start.x, start.y, dir, color, bullet_speed)
    }

    pub fn fire_point(&self) -> Point {
        let x = self.pos.x + self.cells[0].len() as i16 / 2;
        let y = match self.direction {
            Direction::Down => self.pos.y - (self.cells.len() as i16),
            _ => self.pos.y,
        };
        Point::new(x, y)
    }

    pub fn find_char_pos(&self, c: char) -> Vec<Point> {
        let mut v = Vec::new();
        for y in 0..self.cells.len() {
            let row = &self.cells[y];
            for (x, cell) in row.iter().enumerate() {
                if cell.symbol == c.to_string() {
                    v.push(Point::new(x as i16, y as i16));
                }
            }
        }
        v
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
                    cell.symbol,
                    cell.color,
                );
            }
        }
    }
}

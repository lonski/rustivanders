use crate::point::Point;
use crate::renderer::Renderable;

use tui::style::Color;
use tui::widgets::canvas::Context;

pub struct Sprite {
    pub pos: Point,
    pub symbol: &'static str,
    pub color: Color,
}

impl Sprite {
    pub fn new_player(x: i16, y: i16) -> Self {
        Sprite {
            pos: Point::new(x, y),
            symbol: "^",
            color: Color::Blue,
        }
    }

    pub fn new_bullet(x: i16, y: i16) -> Self {
        Sprite {
            pos: Point::new(x, y),
            symbol: "*",
            color: Color::Red,
        }
    }

    pub fn move_by(&mut self, d: &Point) {
        self.pos += *d;
    }
}

impl Renderable for Sprite {
    fn render(&self, ctx: &mut Context) {
        ctx.print(
            self.pos.x as f64,
            self.pos.y as f64,
            &self.symbol,
            self.color,
        );
    }
}

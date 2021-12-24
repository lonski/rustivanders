use crate::point::Point;
use crate::renderer::{Color, Renderable, Renderer};

pub struct Sprite {
    pub pos: Point,
    pub c: char,
    pub color: Color,
}

impl Sprite {
    pub fn new_player(x: i16, y: i16) -> Self {
        Sprite {
            pos: Point::new(x, y),
            c: '^',
            color: Color::Blue,
        }
    }

    pub fn new_bullet(x: i16, y: i16) -> Self {
        Sprite {
            pos: Point::new(x, y),
            c: '.',
            color: Color::Blue,
        }
    }

    pub fn move_by(&mut self, d: &Point) {
        self.pos += *d;
    }
}

impl Renderable for Sprite {
    fn render(&self, renderer: &mut Box<dyn Renderer>) {
        renderer.draw(&self.pos, self.c, self.color);
    }
}

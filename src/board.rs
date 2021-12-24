use crate::point::Point;
use crate::renderer::{Color, Renderable, Renderer};
use crate::sprite::Sprite;

pub struct Board {
    width: u16,
    height: u16,
    player: Sprite,
    bullets: Vec<Sprite>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            width: 80,
            height: 30,
            player: Sprite::new_player(10, 2),
            bullets: vec![Sprite::new_bullet(10, 0)],
        }
    }

    pub fn move_player_by(&mut self, d: &Point) {
        self.player.move_by(d);
    }

    pub fn update(&mut self) {
        // match self.player.color {
        //     Color::Red => self.player.color = Color::Blue,
        //     _ => self.player.color = Color::Red,
        // }
        for bullet in &mut self.bullets {
            bullet.pos += Point::new(0, 1);
            if bullet.pos.y > 30 {
                bullet.pos = Point::new(0, 0)
            }
        }
    }
}

impl Renderable for Board {
    fn render(&self, renderer: &mut Box<dyn Renderer>) {
        for x in 0..self.width {
            for y in 0..self.height {
                renderer.draw(&Point::new(x as i16, y as i16), '.', Color::Grey)
            }
        }
        self.player.render(renderer);
        for bullet in &self.bullets {
            bullet.render(renderer);
        }
    }
}

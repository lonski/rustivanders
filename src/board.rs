use crate::point::Point;
use crate::renderer::Renderable;
use crate::sprite::Sprite;

use tui::widgets::canvas::Context;

pub struct Board {
    player: Sprite,
    bullets: Vec<Sprite>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            player: Sprite::new_player(10, 2),
            bullets: vec![Sprite::new_bullet(10, 0)],
        }
    }

    pub fn move_player_by(&mut self, d: &Point) {
        self.player.move_by(d);
    }

    pub fn update(&mut self) {
        for bullet in &mut self.bullets {
            bullet.pos += Point::new(0, 1);
            if bullet.pos.y > 30 {
                bullet.pos = Point::new(bullet.pos.x, 0)
            }
        }
    }
}

impl Renderable for Board {
    fn render(&self, ctx: &mut Context) {
        self.player.render(ctx);
        for bullet in &self.bullets {
            bullet.render(ctx);
        }
    }
}

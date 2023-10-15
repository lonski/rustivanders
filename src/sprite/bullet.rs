use crate::ai::BulletAi;
use crate::board::UpdateCommand;
use crate::sprite::Cell;
use crate::sprite::Sprite;
use crate::sprite::SpriteState;
use crate::util::{Direction, Point};
use tui::style::Color;

pub struct Bullet {
    pub state: SpriteState,
    pub ai: BulletAi,
}

impl Bullet {
    pub fn new(x: i16, y: i16, dir: Direction, color: Color, bullet_speed: i32) -> Self {
        Bullet {
            state: SpriteState {
                id: 0,
                hp: 1,
                pos: Point::new(x, y),
                direction: dir,
                cells: vec![vec![Cell::new("*", color)]],
            },
            ai: BulletAi::new(bullet_speed),
        }
    }
}

impl<'a> Sprite<'a> for Bullet {
    fn update(&mut self) -> Vec<UpdateCommand> {
        self.ai.update(&mut self.state)
    }

    fn state(&'a self) -> &'a SpriteState {
        &self.state
    }

    fn set_id(&mut self, id: u32) {
        self.state.id = id;
    }
}

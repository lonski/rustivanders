use crate::sprite::Boss;
use crate::sprite::Invander;
use crate::sprite::Sprite;
use std::collections::HashMap;

pub const SCREEN_WIDTH: usize = 94;
pub const SCREEN_HEIGHT: usize = 30;

pub enum SpriteCategory {
    PlayerBullet,
    AlienBullet,
    Alien,
}

pub struct Level {
    pub number: u16,
    pub player_bullets: HashMap<u32, Box<dyn for<'a> Sprite<'a>>>,
    pub bullets: HashMap<u32, Box<dyn for<'a> Sprite<'a>>>,
    pub aliens: HashMap<u32, Box<dyn for<'a> Sprite<'a>>>,
    pub entity_id_counter: u32,
}

impl Level {
    pub fn new(number: u16) -> Self {
        Level {
            number,
            player_bullets: HashMap::new(),
            bullets: HashMap::new(),
            aliens: HashMap::new(),
            entity_id_counter: 0,
        }
    }

    pub fn one() -> Self {
        let mut level = Level::new(1);

        for row in 0..3 {
            let n = 9;
            for i in 0..n {
                let x = 10 * (i) as i16 + 2;
                let y = (SCREEN_HEIGHT - 2) as i16 - row * 5;
                // let x_max = BOARD_WIDTH as i16 - x * (n - i);
                let x_max = x + 7;
                let x_range = (x, x_max as i16 - 2);
                level.add_sprite(
                    Box::new(Invander::new_fighter(x, y, &x_range)),
                    SpriteCategory::Alien,
                );
            }
        }

        level
    }

    pub fn two() -> Self {
        let mut level = Level::new(2);

        for row in 0..3 {
            let n = 9;
            for i in 0..n {
                let x = 10 * (i) as i16 + 2;
                let y = (SCREEN_HEIGHT - 2) as i16 - row * 5;
                // let x_max = BOARD_WIDTH as i16 - x * (n - i);
                let x_max = x + 7;
                let x_range = (x, x_max as i16 - 2);
                if row == 2 {
                    level.add_sprite(
                        Box::new(Invander::new_tank(x, y, &x_range)),
                        SpriteCategory::Alien,
                    );
                } else {
                    level.add_sprite(
                        Box::new(Invander::new_fighter(x, y, &x_range)),
                        SpriteCategory::Alien,
                    );
                }
            }
        }

        level
    }

    pub fn three() -> Self {
        let mut level = Level::new(3);

        for row in 0..5 {
            let x = -10 * row;
            let y = (SCREEN_HEIGHT - 2) as i16 - row * 5;
            let x_max = SCREEN_WIDTH as i16 + (10 * row);
            let x_range = (x, x_max as i16 - 2);
            level.add_sprite(
                Box::new(Invander::new_assasin(x, y, &x_range)),
                SpriteCategory::Alien,
            );
        }

        level
    }

    pub fn four() -> Self {
        let mut level = Level::new(4);

        let x = 32;
        let y = 25;
        let x_range = (-40, SCREEN_WIDTH as i16 + 20);
        let y_range = (20, 30);
        level.add_sprite(
            Box::new(Boss::new_boss(x, y, &x_range, &y_range)),
            SpriteCategory::Alien,
        );

        level
    }

    pub fn is_finished(&self) -> bool {
        self.aliens.is_empty()
    }

    fn next_id(&mut self) -> u32 {
        self.entity_id_counter += 1;
        self.entity_id_counter
    }

    pub fn add_sprite(
        &mut self,
        mut sprite: Box<dyn for<'a> Sprite<'a>>,
        category: SpriteCategory,
    ) {
        let id = self.next_id();
        sprite.set_id(id);
        match category {
            SpriteCategory::Alien => {
                self.aliens.insert(id, sprite);
            }
            SpriteCategory::AlienBullet => {
                self.bullets.insert(id, sprite);
            }
            SpriteCategory::PlayerBullet => {
                self.player_bullets.insert(id, sprite);
            }
        }
    }
}

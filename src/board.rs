use crate::renderer::Renderable;
use crate::sprite::{Bullet, Invander, Player, Sprite};
use crate::util::Direction;

use std::collections::HashMap;
use tui::style::Color;
use tui::widgets::canvas::Context;

pub const BOARD_WIDTH: usize = 94;
pub const BOARD_HEIGHT: usize = 30;

pub enum UpdateCommand {
    SpawnBullet(Bullet),
    SpawnPlayerBullet(Bullet),
    RemoveBullet(u32),
    RemoveInvander(u32),
}

enum SpriteCategory {
    PlayerBullet,
    AlienBullet,
    Alien,
}

pub struct Board {
    pub player: Player,
    pub player_bullets: HashMap<u32, Box<dyn for<'a> Sprite<'a>>>,
    pub bullets: HashMap<u32, Box<dyn for<'a> Sprite<'a>>>,
    pub aliens: HashMap<u32, Box<dyn for<'a> Sprite<'a>>>,
    pub entity_id_counter: u32,
    pub game_over: bool,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            game_over: false,
            player: Player::new((BOARD_WIDTH / 2) as i16, 1),
            player_bullets: HashMap::new(),
            bullets: HashMap::new(),
            aliens: HashMap::new(),
            entity_id_counter: 0,
        };
        for row in 0..3 {
            let n = 9;
            for i in 0..n {
                let x = 10 * (i) as i16 + 2;
                let y = (BOARD_HEIGHT - 2) as i16 - row * 5;
                // let x_max = BOARD_WIDTH as i16 - x * (n - i);
                let x_max = x + 7;
                let x_range = (x, x_max as i16);
                if row == 2 {
                    board.add_sprite(
                        Box::new(Invander::new_small(x, y, &x_range)),
                        SpriteCategory::Alien,
                    );
                } else {
                    board.add_sprite(
                        Box::new(Invander::new(x, y, &x_range)),
                        SpriteCategory::Alien,
                    );
                }
            }
        }
        board
    }

    pub fn move_player(&mut self, dir: Direction) {
        self.player.ai.do_move = dir;
    }

    pub fn player_fire(&mut self) {
        self.player.ai.do_fire = true;
    }

    pub fn update(&mut self) {
        if self.game_over || self.aliens.len() == 0 {
            return;
        }

        let after_update_commands = Board::update_sprites(self.bullets.values_mut());
        self.execute_update_commands(after_update_commands);

        let after_update_commands = Board::update_sprites(self.player_bullets.values_mut());
        self.execute_update_commands(after_update_commands);

        let after_update_commands = Board::update_sprites(self.aliens.values_mut());
        self.execute_update_commands(after_update_commands);

        let after_update_commands = self.player.ai.update(&mut self.player.state);
        self.execute_update_commands(after_update_commands);

        let mut kill_aliens: Vec<UpdateCommand> = Vec::new();
        for (bullet_id, bullet) in &self.player_bullets {
            for (alien_id, alien) in &self.aliens {
                if alien.collides(&bullet.state().pos) {
                    kill_aliens.push(UpdateCommand::RemoveInvander(*alien_id));
                    kill_aliens.push(UpdateCommand::RemoveBullet(*bullet_id));
                    break;
                }
            }
        }
        self.execute_update_commands(kill_aliens);

        for (_, bullet) in &self.bullets {
            if self.player.state.collides(&bullet.state().pos) {
                self.game_over = true;
            }
        }
    }

    fn update_sprites<'b, I>(sprites: I) -> Vec<UpdateCommand>
    where
        I: Iterator<Item = &'b mut Box<dyn for<'a> Sprite<'a>>>,
    {
        sprites.flat_map(|e| e.update()).collect::<Vec<_>>()
    }

    fn next_id(&mut self) -> u32 {
        self.entity_id_counter += 1;
        self.entity_id_counter
    }

    fn add_sprite(&mut self, mut sprite: Box<dyn for<'a> Sprite<'a>>, category: SpriteCategory) {
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

    fn execute_update_commands(&mut self, commands: Vec<UpdateCommand>) {
        for cmd in commands {
            match cmd {
                UpdateCommand::SpawnBullet(sprite) => {
                    self.add_sprite(Box::new(sprite), SpriteCategory::AlienBullet)
                }
                UpdateCommand::SpawnPlayerBullet(sprite) => {
                    self.add_sprite(Box::new(sprite), SpriteCategory::PlayerBullet)
                }
                UpdateCommand::RemoveBullet(id) => {
                    self.bullets.remove(&id);
                    self.player_bullets.remove(&id);
                }
                UpdateCommand::RemoveInvander(id) => {
                    self.aliens.remove(&id);
                }
            }
        }
    }
}

impl Renderable for Board {
    fn render(&self, ctx: &mut Context) {
        self.player.state.render(ctx);
        for (_, bullet) in &self.bullets {
            bullet.render(ctx);
        }
        for (_, bullet) in &self.player_bullets {
            bullet.render(ctx);
        }
        for (_, invander) in &self.aliens {
            invander.render(ctx);
        }
        if self.game_over {
            ctx.print(
                BOARD_WIDTH as f64 / 2.0 - 5.0,
                BOARD_HEIGHT as f64 / 2.5,
                "GAME OVER",
                Color::Red,
            );
        }

        if self.aliens.len() == 0 {
            ctx.print(
                BOARD_WIDTH as f64 / 2.0 - 4.0,
                BOARD_HEIGHT as f64 / 2.0,
                "You won!",
                Color::Green,
            );
        }
    }
}

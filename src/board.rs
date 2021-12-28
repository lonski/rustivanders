use crate::level::{Level, SpriteCategory, SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::renderer::Renderable;
use crate::sprite::{Bullet, Player, Sprite};
use crate::util::Direction;

use tui::style::Color;
use tui::widgets::canvas::Context;

pub enum UpdateCommand {
    SpawnBullet(Bullet),
    SpawnPlayerBullet(Bullet),
    RemoveBullet(u32),
    RemoveInvander(u32),
}

pub struct Board {
    pub player: Player,
    pub level: Level,
    pub game_over: bool,
}

impl Board {
    pub fn new() -> Self {
        Board {
            game_over: false,
            player: Player::new((SCREEN_WIDTH / 2) as i16, 1),
            level: Level::one(),
        }
    }

    pub fn move_player(&mut self, dir: Direction) {
        self.player.ai.do_move = dir;
    }

    pub fn player_fire(&mut self) {
        self.player.ai.do_fire = true;
    }

    fn reset_game_with_level(&mut self, level: Level) {
        self.level = level;
        self.game_over = false;
    }

    pub fn next_level(&mut self) {
        if self.level.is_finished() {
            match self.level.number {
                1 => self.reset_game_with_level(Level::two()),
                2 => self.reset_game_with_level(Level::three()),
                3 => self.reset_game_with_level(Level::four()),
                _ => self.reset_game_with_level(Level::one()),
            }
        } else if self.game_over {
            self.reset_game_with_level(Level::one());
        }
    }

    pub fn update(&mut self) {
        if self.game_over || self.level.is_finished() {
            return;
        }

        // Update sprites
        let mut after_update_commands = [
            Board::update_sprites(self.level.bullets.values_mut()),
            Board::update_sprites(self.level.player_bullets.values_mut()),
            Board::update_sprites(self.level.aliens.values_mut()),
            self.player.ai.update(&mut self.player.state),
        ]
        .into_iter()
        .flat_map(|u| u.into_iter())
        .collect::<Vec<_>>();

        // Check collisions with aliens
        for (bullet_id, bullet) in &self.level.player_bullets {
            for (alien_id, alien) in &mut self.level.aliens {
                if alien.collides(&bullet.state().pos) {
                    after_update_commands.push(UpdateCommand::RemoveBullet(*bullet_id));
                    alien.modify_hp(-1);
                    if alien.state().hp == 0 {
                        after_update_commands.push(UpdateCommand::RemoveInvander(*alien_id));
                    }
                    break;
                }
            }
        }
        self.execute_update_commands(after_update_commands);

        // Check collistions with player
        for bullet in self.level.bullets.values() {
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

    fn execute_update_commands(&mut self, commands: Vec<UpdateCommand>) {
        for cmd in commands {
            match cmd {
                UpdateCommand::SpawnBullet(sprite) => self
                    .level
                    .add_sprite(Box::new(sprite), SpriteCategory::AlienBullet),
                UpdateCommand::SpawnPlayerBullet(sprite) => self
                    .level
                    .add_sprite(Box::new(sprite), SpriteCategory::PlayerBullet),
                UpdateCommand::RemoveBullet(id) => {
                    self.level.bullets.remove(&id);
                    self.level.player_bullets.remove(&id);
                }
                UpdateCommand::RemoveInvander(id) => {
                    self.level.aliens.remove(&id);
                }
            }
        }
    }
}

impl Renderable for Board {
    fn render(&self, ctx: &mut Context) {
        self.player.state.render(ctx);
        for bullet in self.level.bullets.values() {
            bullet.render(ctx);
        }
        for bullet in self.level.player_bullets.values() {
            bullet.render(ctx);
        }
        for invander in self.level.aliens.values() {
            invander.render(ctx);
        }
        if self.game_over {
            let x = SCREEN_WIDTH as f64 / 2.0 - 5.0;
            let y = SCREEN_HEIGHT as f64 / 2.5;
            ctx.print(x, y, "GAME OVER", Color::Red);
            ctx.print(
                x - 9.0,
                y - 1.0,
                "Press 'n' to start new game",
                Color::LightRed,
            );
        }

        if self.level.is_finished() {
            let x = SCREEN_WIDTH as f64 / 2.0 - 4.0;
            let y = SCREEN_HEIGHT as f64 / 2.0;
            ctx.print(x, y, "You won!", Color::Green);
            ctx.print(
                x - 8.0,
                y - 1.0,
                "Press 'n' for next level",
                Color::LightGreen,
            );
        }
    }
}

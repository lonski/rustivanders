use crate::renderer::Renderable;
use crate::sprite::{Bullet, Direction, Invander, Player, Sprite};

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

pub struct BoardState {
    pub player: Player,
    pub player_bullets: HashMap<u32, Box<dyn for<'a> Sprite<'a>>>,
    pub bullets: HashMap<u32, Box<dyn for<'a> Sprite<'a>>>,
    pub invanders: HashMap<u32, Box<dyn for<'a> Sprite<'a>>>,
    pub entity_id_counter: u32,
    pub game_over: bool,
}

impl BoardState {
    fn next_id(&mut self) -> u32 {
        self.entity_id_counter += 1;
        self.entity_id_counter
    }

    pub fn add_bullet(&mut self, bullet: Bullet) {
        let mut s = bullet;
        s.state.id = self.next_id();
        self.bullets.insert(s.state.id, Box::new(s));
    }

    pub fn add_player_bullet(&mut self, bullet: Bullet) {
        let mut s = bullet;
        s.state.id = self.next_id();
        self.player_bullets.insert(s.state.id, Box::new(s));
    }

    pub fn add_invander(&mut self, invander: Invander) {
        let mut s = invander;
        s.state.id = self.next_id();
        self.invanders.insert(s.state.id, Box::new(s));
    }
}

pub struct Board {
    pub state: BoardState,
}

impl Board {
    pub fn new() -> Self {
        let mut board = Board {
            state: BoardState {
                game_over: false,
                player: Player::new((BOARD_WIDTH / 2) as i16, 1),
                player_bullets: HashMap::new(),
                bullets: HashMap::new(),
                invanders: HashMap::new(),
                entity_id_counter: 0,
            },
        };
        for row in 0..3 {
            let n = 7;
            for i in 0..n {
                let x = 10 * (i + 1) as i16;
                let y = (BOARD_HEIGHT - 2) as i16 - row * 5;
                // let x_max = BOARD_WIDTH as i16 - x * (n - i);
                let x_max = x + 10;
                let x_range = (x, x_max as i16);
                board.state.add_invander(Invander::new(x, y, &x_range));
            }
        }
        board
    }

    pub fn move_player(&mut self, dir: Direction) {
        self.state.player.ai.do_move = dir;
    }

    pub fn player_fire(&mut self) {
        self.state.player.ai.do_fire = true;
    }

    pub fn update(&mut self) {
        if self.state.game_over || self.state.invanders.len() == 0 {
            return;
        }
        let updates = self
            .state
            .bullets
            .iter_mut()
            .flat_map(|(_, e)| e.update())
            .collect::<Vec<_>>();
        self.execute_update_commands(updates);

        let updates = self
            .state
            .player_bullets
            .iter_mut()
            .flat_map(|(_, e)| e.update())
            .collect::<Vec<_>>();
        self.execute_update_commands(updates);

        let updates = self
            .state
            .invanders
            .iter_mut()
            .flat_map(|(_, e)| e.update())
            .collect::<Vec<_>>();
        self.execute_update_commands(updates);

        let updates = self.state.player.ai.update(&mut self.state.player.state);
        self.execute_update_commands(updates);

        let mut kill_aliens: Vec<UpdateCommand> = Vec::new();
        for (bullet_id, bullet) in &self.state.player_bullets {
            for (alien_id, alien) in &self.state.invanders {
                if alien.collides(&bullet.state().pos) {
                    kill_aliens.push(UpdateCommand::RemoveInvander(*alien_id));
                    kill_aliens.push(UpdateCommand::RemoveBullet(*bullet_id));
                    break;
                }
            }
        }
        self.execute_update_commands(kill_aliens);

        for (_, bullet) in &self.state.bullets {
            if self.state.player.state.collides(&bullet.state().pos) {
                self.state.game_over = true;
            }
        }
    }

    fn execute_update_commands(&mut self, commands: Vec<UpdateCommand>) {
        for cmd in commands {
            match cmd {
                UpdateCommand::SpawnBullet(sprite) => self.state.add_bullet(sprite),
                UpdateCommand::SpawnPlayerBullet(sprite) => self.state.add_player_bullet(sprite),
                UpdateCommand::RemoveBullet(id) => {
                    self.state.bullets.remove(&id);
                    self.state.player_bullets.remove(&id);
                }
                UpdateCommand::RemoveInvander(id) => {
                    self.state.invanders.remove(&id);
                }
            }
        }
    }
}

impl Renderable for Board {
    fn render(&self, ctx: &mut Context) {
        self.state.player.state.render(ctx);
        for (_, bullet) in &self.state.bullets {
            bullet.state().render(ctx);
        }
        for (_, bullet) in &self.state.player_bullets {
            bullet.state().render(ctx);
        }
        for (_, invander) in &self.state.invanders {
            invander.state().render(ctx);
        }
        if self.state.game_over {
            ctx.print(
                BOARD_WIDTH as f64 / 2.0 - 5.0,
                BOARD_HEIGHT as f64 / 2.5,
                "GAME OVER",
                Color::Red,
            );
        }

        if self.state.invanders.len() == 0 {
            ctx.print(
                BOARD_WIDTH as f64 / 2.0 - 4.0,
                BOARD_HEIGHT as f64 / 2.0,
                "You won!",
                Color::Green,
            );
        }
    }
}

use crate::board::{UpdateCommand, BOARD_HEIGHT, BOARD_WIDTH};
use crate::point::Point;
use crate::sprite::{Direction, SpriteState};
use tui::style::Color;

pub struct BulletAi {}

impl BulletAi {
    pub fn update(&mut self, sprite: &mut SpriteState) -> Vec<UpdateCommand> {
        let mut cmds = Vec::new();
        match sprite.direction {
            Direction::Up => {
                sprite.move_by(&Point::new(0, 1));
            }
            _ => {
                sprite.move_by(&Point::new(0, -1));
            }
        }
        if sprite.pos.y < 0 || sprite.pos.y > BOARD_HEIGHT as i16 {
            cmds.push(UpdateCommand::RemoveBullet(sprite.id));
        }
        cmds
    }
}

pub struct PlayerAi {
    pub ticks_to_fire: u32,
    pub do_fire: bool,
    pub do_move: Direction,
}

impl PlayerAi {
    pub fn new() -> Self {
        PlayerAi {
            do_fire: false,
            do_move: Direction::None,
            ticks_to_fire: 15,
        }
    }

    pub fn update(&mut self, sprite: &mut SpriteState) -> Vec<UpdateCommand> {
        let mut cmd = Vec::new();
        if self.do_fire && self.ticks_to_fire == 0 {
            self.do_fire = false;
            self.ticks_to_fire = 15;
            cmd.push(UpdateCommand::SpawnPlayerBullet(
                sprite.fire(Direction::Up, Color::LightBlue),
            ));
        }
        if self.ticks_to_fire > 0 {
            self.ticks_to_fire -= 1;
        }
        match self.do_move {
            Direction::Left => {
                if sprite.pos.x <= 1 {
                    self.do_move = Direction::None;
                } else {
                    sprite.move_by(&Point::new(-1, 0));
                }
            }
            Direction::Right => {
                if sprite.pos.x >= (BOARD_WIDTH - 2) as i16 {
                    self.do_move = Direction::None;
                } else {
                    sprite.move_by(&Point::new(1, 0));
                }
            }
            _ => {}
        }
        cmd
    }
}

#[derive(Clone)]
pub struct InvanderAi {
    pub move_dir: Direction,
    pub ticks_to_spawn_bullet: u16,
    pub ticks_to_move: u16,
    pub x_range: (i16, i16),
}

impl InvanderAi {
    pub fn new(x_range: &(i16, i16)) -> Self {
        InvanderAi {
            x_range: *x_range,
            move_dir: Direction::Left,
            ticks_to_spawn_bullet: InvanderAi::random_tick_to_spawn_bullet(),
            ticks_to_move: 3,
        }
    }

    fn random_tick_to_spawn_bullet() -> u16 {
        (rand::random::<f64>() * 200.0 + 150.0) as u16
    }
}

impl InvanderAi {
    pub fn update(&mut self, sprite: &mut SpriteState) -> Vec<UpdateCommand> {
        let mut commands: Vec<UpdateCommand> = Vec::new();
        self.ticks_to_move -= 1;
        if self.ticks_to_move == 0 {
            self.ticks_to_move = 3;
            match self.move_dir {
                Direction::Left => {
                    sprite.move_by(&Point::new(-1, 0));
                    if sprite.pos.x < self.x_range.0 as i16 {
                        self.move_dir = Direction::Right;
                    }
                }
                _ => {
                    sprite.move_by(&Point::new(1, 0));
                    if sprite.pos.x > self.x_range.1 {
                        self.move_dir = Direction::Left;
                    }
                }
            }
        }
        self.ticks_to_spawn_bullet -= 1;
        if self.ticks_to_spawn_bullet == 0 {
            self.ticks_to_spawn_bullet = InvanderAi::random_tick_to_spawn_bullet();
            commands.push(UpdateCommand::SpawnBullet(
                sprite.fire(Direction::Down, Color::Red),
            ));
        }

        commands
    }
}

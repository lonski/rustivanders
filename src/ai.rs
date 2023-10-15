use crate::board::UpdateCommand;
use crate::level::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::sprite::{Bullet, SpriteState};
use crate::util::{Direction, Point};

use tui::style::Color;

pub struct BulletAi {
    pub speed: i32,
    pub tick: i32,
}

impl BulletAi {
    pub fn new(speed: i32) -> Self {
        BulletAi { speed, tick: 0 }
    }

    pub fn update(&mut self, sprite: &mut SpriteState) -> Vec<UpdateCommand> {
        let mut cmds = Vec::new();
        self.tick += 1;
        if self.tick >= self.speed {
            self.tick = 0;

            match sprite.direction {
                Direction::Up => {
                    sprite.move_by(&Point::new(0, 1));
                }
                _ => {
                    sprite.move_by(&Point::new(0, -1));
                }
            }
            if sprite.pos.y < 0 || sprite.pos.y > SCREEN_HEIGHT as i16 {
                cmds.push(UpdateCommand::RemoveBullet(sprite.id));
            }
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
            cmd.push(UpdateCommand::SpawnPlayerBullet(sprite.fire(
                Direction::Up,
                Color::LightBlue,
                1,
            )));
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
                if sprite.pos.x >= (SCREEN_WIDTH - 2) as i16 {
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

pub struct InvanderAi {
    pub move_dir: Direction,
    pub ticks_to_spawn_bullet: u16,
    pub ticks_to_move: u16,
    pub x_range: (i16, i16),
    pub move_speed: u16,
    pub fire_rate: f64,
    pub bullet_speed: i32,
}

impl InvanderAi {
    pub fn new(x_range: &(i16, i16), move_speed: u16, fire_rate: f64, bullet_speed: i32) -> Self {
        let mut alien = InvanderAi {
            x_range: *x_range,
            move_dir: Direction::Left,
            move_speed,
            ticks_to_move: move_speed,
            fire_rate,
            ticks_to_spawn_bullet: 0,
            bullet_speed,
        };
        alien.random_tick_to_spawn_bullet();
        alien
    }

    fn random_tick_to_spawn_bullet(&mut self) {
        self.ticks_to_spawn_bullet = (rand::random::<f64>() * 100.0 * self.fire_rate) as u16;
    }
}

impl InvanderAi {
    pub fn update(&mut self, sprite: &mut SpriteState) -> Vec<UpdateCommand> {
        let mut commands: Vec<UpdateCommand> = Vec::new();
        self.ticks_to_move -= 1;
        if self.ticks_to_move == 0 {
            self.ticks_to_move = self.move_speed;
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
        if self.fire_rate > 0.0 {
            self.ticks_to_spawn_bullet -= 1;
            if self.ticks_to_spawn_bullet == 0 {
                self.random_tick_to_spawn_bullet();
                commands.push(UpdateCommand::SpawnBullet(sprite.fire(
                    Direction::Down,
                    Color::Red,
                    self.bullet_speed,
                )));
            }
        }

        commands
    }
}

pub struct BossAi {
    pub move_dir: Direction,
    pub ticks_to_spawn_bullet: u16,
    pub ticks_to_move: u16,
    pub x_range: (i16, i16),
    pub y_range: (i16, i16),
    pub move_speed: u16,
    pub fire_rate: f64,
    pub bullet_count: u16,
}

impl BossAi {
    pub fn new(
        x_range: &(i16, i16),
        y_range: &(i16, i16),
        move_speed: u16,
        fire_rate: f64,
    ) -> Self {
        let mut alien = BossAi {
            bullet_count: 10,
            x_range: *x_range,
            y_range: *y_range,
            move_dir: Direction::Left,
            move_speed,
            ticks_to_move: move_speed,
            fire_rate,
            ticks_to_spawn_bullet: 0,
        };
        alien.random_tick_to_spawn_bullet();
        alien
    }

    fn random_tick_to_spawn_bullet(&mut self) {
        self.ticks_to_spawn_bullet = (rand::random::<f64>() * 100.0 * self.fire_rate) as u16;
    }

    fn random_v_dir(&self) -> Direction {
        if rand::random::<f64>() > 0.5 {
            return Direction::Up;
        }
        Direction::Down
    }
}

impl BossAi {
    fn calc_y_mod(&self, sprite: &SpriteState) -> i16 {
        if (sprite.pos.x == -35 && self.move_dir == Direction::Left)
            || (sprite.pos.x == SCREEN_WIDTH as i16 + 35 && self.move_dir == Direction::Right)
        {
            return match self.random_v_dir() {
                Direction::Down => {
                    -((std::cmp::max(sprite.pos.y - self.y_range.0, 0) as f64
                        * rand::random::<f64>()) as i16)
                }
                _ => {
                    (std::cmp::max(self.y_range.1 - sprite.pos.y, 0) as f64 * rand::random::<f64>())
                        as i16
                }
            };
        };

        0
    }

    pub fn update(&mut self, sprite: &mut SpriteState) -> Vec<UpdateCommand> {
        let mut commands: Vec<UpdateCommand> = Vec::new();
        self.ticks_to_move -= 1;
        if self.ticks_to_move == 0 {
            self.ticks_to_move = self.move_speed;
            match self.move_dir {
                Direction::Left => {
                    sprite.move_by(&Point::new(-1, self.calc_y_mod(sprite)));
                    if sprite.pos.x < self.x_range.0 as i16 {
                        self.move_dir = Direction::Right;
                    }
                }
                _ => {
                    sprite.move_by(&Point::new(1, self.calc_y_mod(sprite)));
                    if sprite.pos.x > self.x_range.1 {
                        self.move_dir = Direction::Left;
                    }
                }
            }
        }
        if self.fire_rate > 0.0 {
            if self.ticks_to_spawn_bullet == 0 && self.bullet_count > 0 {
                for laser in sprite.find_char_pos('V') {
                    commands.push(UpdateCommand::SpawnBullet(Bullet::new(
                        laser.x + sprite.pos.x,
                        -laser.y + sprite.pos.y,
                        Direction::Down,
                        Color::LightMagenta,
                        1,
                    )));
                }
                self.bullet_count -= 1;
            } else if self.bullet_count == 0 {
                self.bullet_count = 10;
                self.random_tick_to_spawn_bullet();
            } else {
                self.ticks_to_spawn_bullet -= 1;
            }
        }

        commands
    }
}

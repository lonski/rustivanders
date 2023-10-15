use crate::ai::InvanderAi;
use crate::board::UpdateCommand;
use crate::sprite::Cell;
use crate::sprite::Sprite;
use crate::sprite::SpriteState;
use crate::util::{Direction, Point};
use tui::style::Color;

pub struct Invander {
    pub state: SpriteState,
    pub ai: InvanderAi,
}

impl Invander {
    pub fn new_fighter(x: i16, y: i16, x_range: &(i16, i16)) -> Self {
        Invander {
            state: SpriteState {
                id: 0,
                hp: 1,
                pos: Point::new(x, y),
                direction: Direction::Down,
                cells: vec![
                    vec![
                        Cell::new(" ", Color::Green),
                        Cell::new("_", Color::Green),
                        Cell::new("_", Color::Green),
                        Cell::new("_", Color::Green),
                        Cell::new("_", Color::Green),
                        Cell::new("_", Color::Green),
                        Cell::new(" ", Color::Green),
                    ],
                    vec![
                        Cell::new("<", Color::Green),
                        Cell::new(":", Color::Yellow),
                        Cell::new(":", Color::Yellow),
                        Cell::new(":", Color::Yellow),
                        Cell::new(":", Color::Yellow),
                        Cell::new(":", Color::Yellow),
                        Cell::new(">", Color::Green),
                    ],
                    vec![
                        Cell::new(" ", Color::Green),
                        Cell::new("|", Color::Red),
                        Cell::new(" ", Color::Green),
                        Cell::new("|", Color::Red),
                        Cell::new(" ", Color::Green),
                        Cell::new("|", Color::Red),
                        Cell::new(" ", Color::Green),
                    ],
                ],
            },
            ai: InvanderAi::new(x_range, 10, 10.0, 8),
        }
    }

    pub fn new_tank(x: i16, y: i16, x_range: &(i16, i16)) -> Self {
        Invander {
            state: SpriteState {
                id: 0,
                hp: 2,
                pos: Point::new(x, y),
                direction: Direction::Down,
                cells: vec![
                    vec![
                        Cell::new(" ", Color::Red),
                        Cell::new("^", Color::Red),
                        Cell::new("^", Color::Red),
                        Cell::new("^", Color::Red),
                        Cell::new(" ", Color::Red),
                    ],
                    vec![
                        Cell::new("{", Color::Green),
                        Cell::new(" ", Color::Green),
                        Cell::new(" ", Color::Green),
                        Cell::new(" ", Color::Green),
                        Cell::new("}", Color::Green),
                    ],
                    vec![
                        Cell::new(" ", Color::Green),
                        Cell::new("V", Color::Yellow),
                        Cell::new("V", Color::Yellow),
                        Cell::new("V", Color::Yellow),
                        Cell::new(" ", Color::Green),
                    ],
                ],
            },

            ai: InvanderAi::new(x_range, 4, 0.0, 1),
        }
    }

    pub fn new_assasin(x: i16, y: i16, x_range: &(i16, i16)) -> Self {
        Invander {
            state: SpriteState {
                id: 0,
                hp: 1,
                pos: Point::new(x, y),
                direction: Direction::Down,
                cells: vec![
                    vec![
                        Cell::new(" ", Color::Green),
                        Cell::new("<", Color::LightGreen),
                        Cell::new("<", Color::Green),
                        Cell::new("<", Color::Cyan),
                        Cell::new(" ", Color::Green),
                        Cell::new(">", Color::Cyan),
                        Cell::new(">", Color::Green),
                        Cell::new(">", Color::LightGreen),
                        Cell::new(" ", Color::Green),
                    ],
                    vec![
                        Cell::new("<", Color::LightGreen),
                        Cell::new("<", Color::Green),
                        Cell::new("<", Color::Cyan),
                        Cell::new(" ", Color::Green),
                        Cell::new("&", Color::Red),
                        Cell::new(" ", Color::Green),
                        Cell::new(">", Color::Cyan),
                        Cell::new(">", Color::Green),
                        Cell::new(">", Color::LightGreen),
                    ],
                    vec![
                        Cell::new(" ", Color::Green),
                        Cell::new("<", Color::LightGreen),
                        Cell::new("<", Color::Green),
                        Cell::new("<", Color::Cyan),
                        Cell::new(" ", Color::Green),
                        Cell::new(">", Color::Cyan),
                        Cell::new(">", Color::Green),
                        Cell::new(">", Color::LightGreen),
                        Cell::new(" ", Color::Green),
                    ],
                ],
            },
            ai: InvanderAi::new(x_range, 1, 0.8, 30),
        }
    }
}

impl<'a> Sprite<'a> for Invander {
    fn set_id(&mut self, id: u32) {
        self.state.id = id;
    }

    fn update(&mut self) -> Vec<UpdateCommand> {
        self.ai.update(&mut self.state)
    }

    fn state(&'a self) -> &'a SpriteState {
        &self.state
    }

    fn modify_hp(&mut self, hp_mod: i16) {
        self.state.hp = std::cmp::max(self.state.hp as i16 + hp_mod, 0) as u16;
    }
}

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
    pub fn new(x: i16, y: i16, x_range: &(i16, i16)) -> Self {
        Invander {
            state: SpriteState {
                id: 0,
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
            ai: InvanderAi::new(x_range),
        }
    }

    pub fn new_small(x: i16, y: i16, x_range: &(i16, i16)) -> Self {
        Invander {
            state: SpriteState {
                id: 0,
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
                        Cell::new("|", Color::LightRed),
                        Cell::new("V", Color::Yellow),
                        Cell::new(" ", Color::Green),
                    ],
                ],
            },

            ai: InvanderAi::new(x_range),
        }
    }
}

impl<'a> Sprite<'a> for Invander {
    fn update(&mut self) -> Vec<UpdateCommand> {
        self.ai.update(&mut self.state)
    }

    fn state(&'a self) -> &'a SpriteState {
        &self.state
    }
}

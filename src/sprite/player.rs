use crate::ai::PlayerAi;
use crate::board::UpdateCommand;
use crate::sprite::Cell;
use crate::sprite::Sprite;
use crate::sprite::SpriteState;
use crate::util::{Direction, Point};
use tui::style::Color;

pub struct Player {
    pub state: SpriteState,
    pub ai: PlayerAi,
}

impl Player {
    pub fn new(x: i16, y: i16) -> Self {
        Player {
            state: SpriteState {
                hp: 1,
                id: 0,
                pos: Point::new(x, y),
                direction: Direction::Up,
                cells: vec![
                    vec![
                        Cell::new(" ", Color::Blue),
                        Cell::new("^", Color::LightBlue),
                    ],
                    vec![
                        Cell::new("/", Color::Blue),
                        Cell::new("V", Color::Red),
                        Cell::new("\\", Color::Blue),
                    ],
                ],
            },
            ai: PlayerAi::new(),
        }
    }
}

impl<'a> Sprite<'a> for Player {
    fn set_id(&mut self, id: u32) {
        self.state.id = id;
    }

    fn update(&mut self) -> Vec<UpdateCommand> {
        self.ai.update(&mut self.state)
    }

    fn state(&'a self) -> &'a SpriteState {
        &self.state
    }
}

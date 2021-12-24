use crate::renderer::Color;
use crate::renderer::Renderer;

pub struct Board {
    player_pos: (u16, u16),
}

impl Board {
    pub fn new() -> Self {
        Board { player_pos: (0, 0) }
    }

    pub fn render(&self, renderer: &mut Box<dyn Renderer>) {
        renderer.draw(self.player_pos, '^', Color::Yellow);
    }

    pub fn move_player_by(&mut self, d: (i16, i16)) {
        let new_x = (self.player_pos.0 as i16 + d.0).max(0) as u16;
        let new_y = (self.player_pos.1 as i16 + d.1).max(0) as u16;
        self.player_pos = (new_x, new_y);
    }
}

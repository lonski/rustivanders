mod ai;
mod board;
mod events;
mod game;
mod renderer;
mod sprite;
mod util;

use crate::game::Rustivanders;

fn main() {
    Rustivanders::new().run();
}

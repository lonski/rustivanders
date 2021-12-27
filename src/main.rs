mod ai;
mod board;
mod events;
mod game;
mod level;
mod renderer;
mod sprite;
mod util;

use crate::game::Rustivanders;

fn main() {
    Rustivanders::new().run();
}
